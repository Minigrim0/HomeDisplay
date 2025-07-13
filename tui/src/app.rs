use log::error;
use std::default::Default;
use std::io::{self, ErrorKind};
use std::time::{Duration, SystemTime};

use ratatui::{
    crossterm::event::{self, poll, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout},
    Frame,
};

use homedisplay::settings::Settings;

use crate::currency::CurrencyComponent;
use crate::datetime::DateTimeComponent;
use crate::transports::TransportComponent;
use crate::tui::Tui;
use crate::utilities;
use crate::weather::WeatherComponent;

#[derive(Debug, Default)]
// Application state
pub struct App {
    pub exit: bool,
    pub settings: Settings,
    pub weather: WeatherComponent,
    pub datetime: DateTimeComponent,
    pub currency: CurrencyComponent,
    pub transports: TransportComponent,
}

impl App {
    pub fn with_settings(mut self, settings_file: &str) -> Self {
        let settings = Settings::load_from_file(settings_file).unwrap_or_else(|e| {
            log::error!("Unable to load settings from file: {}. Using default value", e);
            Settings::default()
        });
        self.settings = settings;
        for timezone in self.settings.timezones.iter() {
            self.datetime.push_timezone(timezone.clone());
        }

        self
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        self.force_complete_refresh(); // Initial refresh
        while !self.exit {
            if let Ok(size) = terminal.size() {
                if size.height < 5 || size.width < 30 {
                    log::error!("{}x{} is not big enough", size.width, size.height);
                    return Err(io::Error::new(ErrorKind::Other, "Terminal not big enough"));
                }
            }

            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
            self.update_state()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        let chunks = Layout::horizontal([Constraint::Ratio(1, 3); 3])
            .split(frame.area());

        let middle_split = Layout::vertical([Constraint::Ratio(4, 5), Constraint::Ratio(1, 5)])
            .split(chunks[1]);

        frame.render_widget(&self.weather, chunks[0]);
        frame.render_widget(&self.datetime, middle_split[0]);
        frame.render_widget(&self.currency, middle_split[1]);
        frame.render_widget(&self.transports, chunks[2]);
    }

    /// Updates the state of the application every frame
    fn update_state(&mut self) -> io::Result<()> {
        match SystemTime::now().duration_since(self.weather.last_refresh) {
            Ok(duration) => {
                if duration > self.weather.cooldown {
                    self.weather = utilities::refresh_weather(self.settings.weather.clone(), &self.settings.redis);
                }
            }
            Err(e) => self.weather = WeatherComponent::new(Err(e.to_string())),
        }

        match SystemTime::now().duration_since(self.currency.last_refresh) {
            Ok(duration) => {
                if duration > self.currency.cooldown {
                    self.currency = utilities::refresh_conversion(self.settings.currency.clone(), &self.settings.redis);
                }
            }
            Err(e) => self.currency = CurrencyComponent::new(Err(e.to_string())),
        }

        match SystemTime::now().duration_since(self.transports.last_refresh) {
            Ok(duration) => {
                if duration > self.transports.cooldown {
                    utilities::refresh_sites(&mut self.transports, self.settings.transports.clone(), &self.settings.redis);
                }
            }
            Err(e) => self.transports.departures.error = Some(e.to_string()),
        }

        match SystemTime::now().duration_since(self.weather.last_forecast_change) {
            Ok(duration) => {
                if duration.as_secs() > 5 {
                    self.weather.current_forecast_day = (self.weather.current_forecast_day + 1) % 7;
                    self.weather.last_forecast_change = SystemTime::now();
                }
            },
            Err(e) => {
                error!("Error: {}", e.to_string());
            }
        }

        match SystemTime::now().duration_since(self.datetime.last_offset_change) {
            Ok(duration) => {
                if duration.as_secs() > 5 {
                    self.datetime.advance_timezone();
                }
            },
            Err(e) => {
                error!("Error: {}", e.to_string());
            }
        }

        Ok(())
    }

    fn force_complete_refresh(&mut self) {
        self.weather = utilities::refresh_weather(self.settings.weather.clone(), &self.settings.redis);
        self.currency = utilities::refresh_conversion(self.settings.currency.clone(), &self.settings.redis);
        utilities::refresh_sites(&mut self.transports, self.settings.transports.clone(), &self.settings.redis);
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('r') => self.force_complete_refresh(),
            _ => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if poll(Duration::from_millis(500))? {
            // Poll for events during 500ms -> ~2fps
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }

        Ok(())
    }
}
