use chrono::prelude::{DateTime, Local};
use std::time::{Duration, SystemTime};

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    text::{Line, Text},
    widgets::{
        Block, Paragraph, Widget,
    },
};

use common::models::currency::Conversion;

use crate::utilities::fit_into;

#[derive(Debug)]
pub struct CurrencyComponent {
    pub last_refresh: SystemTime,
    pub conversion: Result<Conversion, String>,
    pub cooldown: Duration,
}

impl Default for CurrencyComponent {
    fn default() -> CurrencyComponent {
        CurrencyComponent {
            last_refresh: SystemTime::now(),
            conversion: Err("No conversion was fetched yet".to_string()),
            cooldown: Duration::from_secs(60 * 60), // Once per hour
        }
    }
}

impl CurrencyComponent {
    pub fn new(conversion: Result<Conversion, String>) -> CurrencyComponent {
        let mut w = CurrencyComponent::default();
        w.last_refresh = SystemTime::now();
        w.conversion = conversion;
        w
    }
}

impl Widget for &CurrencyComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let last_refreshed = Line::from(
            match SystemTime::now().duration_since(self.last_refresh) {
                Ok(duration) => {
                    let minutes = duration.as_secs() / 60;
                    format!(
                        "{} minute{} ago",
                        minutes,
                        if minutes > 1 { "s" } else { "" }
                    )
                }
                Err(e) => format!("Err: {}", e.to_string()),
            },
        );

        let currency_block = Block::new()
            .title_bottom(last_refreshed.centered());

        let currency_text: Text = match &self.conversion {
            Ok(conversion) => {
                let refresh_date = {
                    let date_fetched = DateTime::from_timestamp(conversion.timestamp, 0)
                        .unwrap()
                        .with_timezone(&Local);
                    let date = format!("{}", date_fetched.format("%d/%m/%Y"));
                    let time = format!("{}", date_fetched.format("%H:%M"));
                    format!("last update {date} {time}")
                };

                let mut lines: Vec<Line> = Vec::new();
                for _ in 1..(area.height - 2) / 2 {
                    lines.push(Line::from(""))
                }
                lines.push(Line::from(vec![
                    format!("{:.2} ", conversion.from_currency_amount).bold(),
                    conversion.from_currency.as_str().green(),
                    " = ".into(),
                    format!("{:.2} ", conversion.to_currency_amount).bold(),
                    conversion.to_currency.as_str().green(),
                ]).centered());
                lines.push(Line::from(refresh_date.gray()).centered());

                Text::from(lines)
            }
            Err(e) => {
                let error_lines = fit_into(e.to_string(), (area.width - 2) as usize);
                let mut lines: Vec<Line> = Vec::new();

                for _ in 1..(area.height - error_lines.len() as u16) / 2 {
                    lines.push(Line::from(""))
                }

                lines.push(Line::from("Error !".red().bold()).centered());
                for line in error_lines {
                    lines.push(Line::from(line).yellow().centered());
                }

                Text::from(lines)
            }
        };

        Paragraph::new(currency_text)
            .block(currency_block)
            .render(area, buf);
    }
}
