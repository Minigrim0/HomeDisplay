use std::time::{SystemTime, Duration};
use std::collections::HashMap;
use chrono::prelude::{Local, DateTime, Timelike};

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title}, Block, Borders, Paragraph, Widget
    },
};

use common::models::transports::{Site, Departure};

#[derive(Debug)]
pub struct Departures {
    sites: Vec<Site>,
    departures: HashMap<String, Vec<Departure>>,
    site_errors: HashMap<String, String>,
    last_update: i64,
    time_since_last_update: i64,
    error: Option<String>,
}

#[derive(Debug)]
pub struct TransportComponent {
    pub last_refresh: SystemTime,
    pub departures: Result<Departures, String>,
    pub cooldown: Duration
}

impl TransportComponent {
    pub fn new(weather: Result<Departures, String>) -> TransportComponent {
        let mut w = TransportComponent::default();
        w.last_refresh = SystemTime::now();
        w.departures = weather;
        w
    }
}

impl Default for TransportComponent {
    fn default() -> TransportComponent {
        TransportComponent {
            last_refresh: SystemTime::now(),
            departures: Err("No departures were fetched yet".to_string()),
            cooldown: Duration::from_secs(30 * 60)
        }
    }
}


impl Widget for &TransportComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let last_refreshed = Title::from(Line::from(
            match SystemTime::now().duration_since(self.last_refresh) {
                Ok(duration) => {
                    let minutes = duration.as_secs() / 60;
                    format!("{} minute{} ago", minutes, if minutes > 1 { "s" } else { "" })
                },
                Err(e) => format!("Err: {}", e.to_string())
            }
        ));

        let weather_block = Block::new()
            .borders(Borders::LEFT)
            .title(
                last_refreshed
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::THICK);

        let counter_text: Text = match &self.departures {
            Ok(departure) => {

                let separator = "-".repeat((0.66 * area.width as f32) as usize);

                Text::from(
                vec![
                    Line::from("Departures here").centered()
                ]
            )},
            Err(e) => Text::from(
                vec![
                    Line::from(""),
                    Line::from(
                        "Error !".red().bold(),
                    ).centered(),
                    Line::from(
                        e.to_string().yellow(),
                    ).centered()
                ]
            )
        };

        Paragraph::new(counter_text)
            .block(weather_block)
            .render(area, buf);
    }
}
