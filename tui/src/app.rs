use log::error;
use std::default::Default;
use std::io::{self, ErrorKind};
use std::sync::mpsc;
use std::time::{Duration, SystemTime};

use ratatui::{
    crossterm::event::{self, poll, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout},
    Frame,
};

use homedisplay::settings::Settings;

use crate::async_manager::{AsyncDataManager, DataUpdate, RefreshConfig};
use crate::currency::CurrencyComponent;
use crate::error::TuiError;
use crate::datetime::DateTimeComponent;
use crate::transports::{TransportComponent, Departures};
use crate::tui::Tui;
use crate::weather::WeatherComponent;

#[derive(Debug)]
/// Main application state containing all UI components
pub struct App {
    pub exit: bool,                           // Flag to exit the application
    pub settings: Settings,                   // Application configuration
    pub weather: WeatherComponent,            // Weather display component
    pub datetime: DateTimeComponent,          // Date/time display component
    pub currency: CurrencyComponent,          // Currency conversion component
    pub transports: TransportComponent,       // Transport departure component
    pub data_receiver: Option<mpsc::Receiver<DataUpdate>>, // Channel for async data updates
}

impl Default for App {
    fn default() -> Self {
        Self {
            exit: false,
            settings: Settings::default(),
            weather: WeatherComponent::default(),
            datetime: DateTimeComponent::default(),
            currency: CurrencyComponent::default(),
            transports: TransportComponent::default(),
            data_receiver: None,
        }
    }
}

impl App {
    /// Loads settings from file and configures the application
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

    /// Starts the async data manager and begins background data fetching
    pub fn start_async_manager(&mut self) -> Result<(), TuiError> {
        let mut manager = AsyncDataManager::new()?;
        let config = RefreshConfig::default();
        let receiver = manager.start_background_tasks(self.settings.clone(), config)?;
        self.data_receiver = Some(receiver);
        
        // Store the manager in a way that it won't be dropped
        // For now, we'll let it live until the app exits
        std::mem::forget(manager);
        
        Ok(())
    }

    /// Runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        // Start async data manager
        if let Err(e) = self.start_async_manager() {
            log::error!("Failed to start async data manager: {}", e);
            return Err(io::Error::new(ErrorKind::Other, e.to_string()));
        }
        while !self.exit {
            if let Ok(size) = terminal.size() {
                if size.height < 5 || size.width < 30 {
                    log::error!("{}x{} is not big enough", size.width, size.height);
                    let error = TuiError::TerminalTooSmall { width: size.width, height: size.height };
                    return Err(io::Error::new(ErrorKind::Other, error.to_string()));
                }
            }

            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
            self.update_state()?;
        }
        Ok(())
    }

    /// Renders all components to the terminal frame in a three-column layout
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

    /// Processes any pending async data updates
    fn process_async_updates(&mut self) {
        if let Some(ref receiver) = self.data_receiver {
            // Process all available updates without blocking
            while let Ok(update) = receiver.try_recv() {
                match update {
                    DataUpdate::Weather(result) => {
                        self.weather = WeatherComponent::new(result);
                    }
                    DataUpdate::Currency(result) => {
                        self.currency = CurrencyComponent::new(result);
                    }
                    DataUpdate::Transport(transport_update) => {
                        let departures = Departures {
                            sites: transport_update.sites,
                            departures: transport_update.departures,
                            site_errors: transport_update.site_errors,
                            error: transport_update.error,
                        };
                        self.transports = TransportComponent::new(departures);
                    }
                }
            }
        }
    }

    /// Updates the state of the application every frame
    fn update_state(&mut self) -> io::Result<()> {
        // Process any new data from async tasks
        self.process_async_updates();
        
        // Handle UI-specific updates (forecast cycling, timezone cycling)

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

    /// Forces a complete refresh of all data components
    /// Force refresh is no longer needed as async tasks handle data fetching
    fn force_complete_refresh(&mut self) {
        // Data fetching is now handled by background async tasks
        // This method is kept for compatibility but does nothing
        log::info!("Async data manager will handle data fetching");
    }

    /// Handles keyboard input events
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('r') => self.force_complete_refresh(),
            _ => {}
        }
    }

    /// Polls for and handles terminal events (keyboard input)
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
