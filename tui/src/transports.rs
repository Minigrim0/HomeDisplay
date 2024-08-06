use std::time::{SystemTime, Duration};
use std::collections::HashMap;

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
    pub sites: Vec<Site>,
    pub departures: HashMap<String, Vec<Departure>>,
    pub site_errors: HashMap<String, String>,
    pub error: Option<String>,
}

impl Default for Departures {
    fn default() -> Departures {
        Departures {
            sites: Vec::new(),
            departures: HashMap::new(),
            site_errors: HashMap::new(),
            error: Some("No departures were fetched yet".to_string()),
        }
    }
}

#[derive(Debug)]
pub struct TransportComponent {
    pub last_refresh: SystemTime,
    pub departures: Departures,
    pub cooldown: Duration
}

impl TransportComponent {
    pub fn new(departures: Departures) -> TransportComponent {
        let mut w = TransportComponent::default();
        w.last_refresh = SystemTime::now();
        w.departures = departures;
        w
    }
}

impl Default for TransportComponent {
    fn default() -> TransportComponent {
        TransportComponent {
            last_refresh: SystemTime::now(),
            departures: Departures::default(),
            cooldown: Duration::from_secs(60)
        }
    }
}

impl Widget for &TransportComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let last_refreshed = Title::from(Line::from(
            match SystemTime::now().duration_since(self.last_refresh) {
                Ok(duration) => {
                    let seconds = duration.as_secs();
                    format!("{} second{} ago", seconds, if seconds > 1 { "s" } else { "" })
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

        let counter_text: Text = if let Some(e) = &self.departures.error {
            Text::from(vec![
                Line::from(""),
                Line::from(
                    "Error !".red().bold(),
                ).centered(),
                Line::from(
                    e.to_string().yellow(),
                ).centered()
            ])
        } else {
            let mut lines: Vec<Line> = vec![
                Line::from("Departures").bold().centered().underlined(),
                Line::from(""),
            ];

            for site in &self.departures.sites {
                lines.push(Line::from(format!(" {}", site.name.as_str()).red().bold().underlined()));
                if self.departures.site_errors.contains_key(&site.id) {
                    lines.push(Line::from(format!("Error: {}", self.departures.site_errors[&site.id])))
                }
                for departure in &self.departures.departures[&site.id] {
                    lines.push(Line::from(format!(" {:6} - {} {}", departure.display, departure.line.id, departure.destination)));
                }
                lines.push(Line::from(""));
            }

            Text::from(lines)
        };

        Paragraph::new(counter_text)
            .block(weather_block)
            .render(area, buf);
    }
}
