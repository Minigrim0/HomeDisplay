use std::collections::HashMap;
use std::time::{Duration, SystemTime};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        Block, Borders, Paragraph, Widget,
    },
};

use homedisplay::models::transports::{Departure, Site};

use crate::error::{TuiError, TuiResult};
use crate::utilities::fit_into;

#[derive(Debug)]
/// Container for departure data across multiple transport sites
pub struct Departures {
    pub sites: Vec<Site>,                             // List of transport sites
    pub departures: HashMap<String, Vec<Departure>>,  // Departures grouped by site ID
    pub site_errors: HashMap<String, TuiError>,       // Per-site error messages
    pub error: Option<TuiError>,                      // General error for all sites
}

impl Default for Departures {
    fn default() -> Departures {
        Departures {
            sites: Vec::new(),
            departures: HashMap::new(),
            site_errors: HashMap::new(),
            error: Some(TuiError::TransportFetch("No departures were fetched yet".to_string())),
        }
    }
}

#[derive(Debug)]
/// Transport departure display component
pub struct TransportComponent {
    pub last_refresh: SystemTime,  // Last time departure data was refreshed
    pub departures: Departures,     // Current departure data
    pub cooldown: Duration,         // Time between refresh attempts
}

impl TransportComponent {
    /// Creates a new transport component with the given departure data
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
            cooldown: Duration::from_secs(60),
        }
    }
}

impl Widget for &TransportComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let last_refreshed = Line::from(
            match SystemTime::now().duration_since(self.last_refresh) {
                Ok(duration) => {
                    let seconds = duration.as_secs();
                    format!(
                        "{} second{} ago",
                        seconds,
                        if seconds > 1 { "s" } else { "" }
                    )
                }
                Err(e) => format!("Err: {}", e.to_string()),
            },
        );

        let weather_block = Block::new()
            .borders(Borders::LEFT)
            .title_bottom(last_refreshed.centered())
            .border_set(border::THICK);

        let counter_text: Text = if let Some(e) = &self.departures.error {
            let error_lines = fit_into(e.to_string(), (area.width - 2) as usize);
            let mut lines: Vec<Line> = Vec::new();
            for _ in 1..(area.height - error_lines.len() as u16) / 2 {
                lines.push(Line::from(""))
            }

            lines.push(Line::from("Error !".red().bold()).centered());
            for line in error_lines {
                lines.push(Line::from(line.to_string().yellow()).centered())
            }

            Text::from(lines)
        } else {
            let mut lines: Vec<Line> = vec![
                Line::from("Departures").bold().centered().underlined(),
                Line::from(""),
            ];

            for site in &self.departures.sites {
                lines.push(Line::from(vec![
                    " ".into(),
                    format!("{}", site.name.as_str()).red().bold().underlined(),
                    " 🚂".into(),
                ]));

                if self.departures.site_errors.contains_key(&site.id) {
                    lines.push(Line::from(format!(
                        "Error: {}",
                        self.departures.site_errors[&site.id]
                    )))
                }

                for departure in &self.departures.departures[&site.id] {
                    lines.push(Line::from(vec![
                        format!("   {:6}", departure.display).into(),
                        format!(" - {}", departure.line.id).bold(),
                        format!(" {}", departure.destination).into()
                    ]));
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
