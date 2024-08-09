use log::error;
use ratatui::layout::Rect;
use std::default::Default;
use std::io::{self, ErrorKind};
use std::time::{Duration, SystemTime};

use ratatui::{
    crossterm::event::{self, poll, Event, KeyCode, KeyEvent, KeyEventKind},
    Frame,
};

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
    pub weather: WeatherComponent,
    pub datetime: DateTimeComponent,
    pub currency: CurrencyComponent,
    pub transports: TransportComponent,
}

impl App {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        self.force_complete_refresh(); // Initial refresh
        while !self.exit {
            if let Ok(size) = terminal.size() {
                if size.height < 5 || size.width < 30 {
                    error!("{}x{} is not big enough", size.width, size.height);
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
        let fs: Rect = frame.size();
        frame.render_widget(
            &self.weather,
            Rect {
                width: fs.width / 3,
                height: fs.height,
                x: 0,
                y: 0,
            },
        );
        frame.render_widget(
            &self.datetime,
            Rect {
                width: fs.width / 3,
                height: fs.height / 2,
                x: fs.width / 3,
                y: 0,
            },
        );
        frame.render_widget(
            &self.currency,
            Rect {
                width: fs.width / 3,
                height: fs.height / 2,
                x: fs.width / 3,
                y: (fs.height / 2) + 1,
            },
        );
        frame.render_widget(
            &self.transports,
            Rect {
                width: fs.width / 3,
                height: fs.height,
                x: 2 * (fs.width / 3),
                y: 0,
            },
        );
    }

    fn update_state(&mut self) -> io::Result<()> {
        match SystemTime::now().duration_since(self.weather.last_refresh) {
            Ok(duration) => {
                if duration > self.weather.cooldown {
                    self.weather = utilities::refresh_weather();
                }
            }
            Err(e) => self.weather = WeatherComponent::new(Err(e.to_string())),
        }

        match SystemTime::now().duration_since(self.currency.last_refresh) {
            Ok(duration) => {
                if duration > self.currency.cooldown {
                    self.currency = utilities::refresh_conversion();
                }
            }
            Err(e) => self.currency = CurrencyComponent::new(Err(e.to_string())),
        }

        match SystemTime::now().duration_since(self.transports.last_refresh) {
            Ok(duration) => {
                if duration > self.transports.cooldown {
                    utilities::refresh_sites(&mut self.transports);
                }
            }
            Err(e) => self.transports.departures.error = Some(e.to_string()),
        }

        Ok(())
    }

    fn force_complete_refresh(&mut self) {
        self.weather = utilities::refresh_weather();
        self.currency = utilities::refresh_conversion();
        utilities::refresh_sites(&mut self.transports);
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
