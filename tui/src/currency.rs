use std::time::{SystemTime, Duration};
use chrono::prelude::{Local, DateTime};

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    text::{Line, Text},
    widgets::{
        block::{Position, Title}, Block, Paragraph, Widget
    },
};

use common::models::currency::Conversion;


#[derive(Debug)]
pub struct CurrencyComponent {
    pub last_refresh: SystemTime,
    pub conversion: Result<Conversion, String>,
    pub cooldown: Duration
}

impl Default for CurrencyComponent {
    fn default() -> CurrencyComponent {
        CurrencyComponent {
            last_refresh: SystemTime::now(),
            conversion: Err("No conversion was fetched yet".to_string()),
            cooldown: Duration::from_secs(60 * 60)  // Once per hour
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
        let last_refreshed = Title::from(Line::from(
            match SystemTime::now().duration_since(self.last_refresh) {
                Ok(duration) => {
                    let minutes = duration.as_secs() / 60;
                    format!("{} minute{} ago", minutes, if minutes > 1 { "s" } else { "" })
                },
                Err(e) => format!("Err: {}", e.to_string())
            }
        ));

        let currency_block = Block::new()
            .title(
                last_refreshed
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            );


        let currency_text: Text = match &self.conversion {
            Ok(conversion) => {
                let refresh_date = {
                    let date_fetched = DateTime::from_timestamp(conversion.timestamp, 0).unwrap().with_timezone(&Local);
                    let date = format!("{}", date_fetched.format("%d/%m/%Y"));
                    let time = format!("{}", date_fetched.format("%H:%M"));
                    format!("last update {date} {time}")
                };

                Text::from(vec![
                    Line::from(vec![
                        format!("{:.2} ", conversion.from_currency_amount).bold(),
                        conversion.from_currency.as_str().green(),
                        " = ".into(),
                        format!("{:.2} ", conversion.to_currency_amount).bold(),
                        conversion.to_currency.as_str().green()
                    ]).centered(),
                    Line::from(refresh_date.gray()).centered()
                ])
            },
            Err(e) => Text::from(
                vec![
                    Line::from(
                        "Error !".red().bold(),
                    ).centered(),
                    Line::from(
                        e.to_string().yellow(),
                    ).centered()
                ]
            )
        };

        Paragraph::new(currency_text)
            .block(currency_block)
            .render(area, buf);
    }
}
