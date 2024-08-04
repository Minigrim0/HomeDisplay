use chrono::prelude::{Local, DateTime, Timelike};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
};


use crate::app::App;

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Home Display ".bold());
        let instructions = Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::THICK);

        let counter_text = match &self.weather.weather {
            Ok(weather) => {
                let sunrise = {
                    let timestamp = weather.sys.sunrise;
                    let sunrise = DateTime::from_timestamp(timestamp, 0).unwrap().with_timezone(&Local);
                    format!("{:02}:{:02}", sunrise.hour(), sunrise.minute())
                };
                let sunset = {
                    let timestamp = weather.sys.sunset;
                    let sunset = DateTime::from_timestamp(timestamp, 0).unwrap().with_timezone(&Local);
                    format!("{:02}:{:02}", sunset.hour(), sunset.minute())
                };

                Text::from(
                vec![
                    Line::from(
                        format!("Temperature: {}°C", weather.main.temp).bold()
                    ),
                    Line::from(
                        format!("Feels like: {}°C", weather.main.feels_like.to_string().yellow())
                    ),
                    Line::from(
                        format!("High: {}°C", weather.main.feels_like.to_string().yellow())
                    ),
                    Line::from(
                        format!("Low: {}°C", weather.main.feels_like.to_string().yellow())
                    ),

                    Line::from(
                        format!("Clouds ?: {}°C", weather.main.feels_like.to_string().yellow())
                    ),

                    Line::from("🌕 Day time ☀️".to_string().yellow()),
                    Line::from(format!("🌅 {} 🌄 {}", sunrise, sunset))
                ]
            )},
            Err(e) => Text::from(
                vec![
                    Line::from(
                        "Error !".red().bold(),
                    ),
                    Line::from(
                        e.to_string().yellow(),
                    )
                ]
            )
        };

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
