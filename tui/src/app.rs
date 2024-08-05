use std::io;
use std::time::{Duration, SystemTime};
use std::default::Default;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, poll},
    Frame,
};

use crate::currency::CurrencyComponent;
use crate::datetime::DateTimeComponent;
use crate::transports::TransportComponent;
use crate::tui::Tui;
use crate::weather::WeatherComponent;
use crate::utilities;

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
        self.force_complete_refresh();
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
            self.update_state()?;
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        let frame_size = {
            let mut fs = frame.size();
            fs.width = fs.width / 3;
            fs
        };
        frame.render_widget(&self.weather, frame_size);
        let frame_size = {
            let mut fs = frame.size();
            fs.width = fs.width / 3;
            fs.height = fs.height / 2;
            fs.x = fs.width;
            fs
        };
        frame.render_widget(&self.datetime, frame_size);
        let frame_size = {
            let mut fs = frame.size();
            fs.width = fs.width / 3;
            fs.height = fs.height / 2;
            fs.x = fs.width;
            fs.y = fs.height + 1;
            fs
        };
        frame.render_widget(&self.currency, frame_size);
        let frame_size = {
            let mut fs = frame.size();
            fs.width = fs.width / 3;
            fs.x = 2 * fs.width;
            fs
        };
        frame.render_widget(&self.transports, frame_size);
    }

    fn update_state(&mut self) -> io::Result<()> {
        match SystemTime::now().duration_since(self.weather.last_refresh) {
            Ok(duration) => if duration > self.weather.cooldown {
                self.weather = utilities::refresh_weather();
            },
            Err(e) => self.weather = WeatherComponent::new(Err(e.to_string()))
        }

        Ok(())
    }

    fn force_complete_refresh(&mut self) {
        self.weather = utilities::refresh_weather();
        self.currency = utilities::refresh_conversion();
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('r') => self.force_complete_refresh(),
            _ => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if poll(Duration::from_millis(100))? {
            match event::read()? {
                // it's important to check that the event is a key press event as
                // crossterm also emits key release and repeat events on Windows.
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }

        Ok(())
    }
}
