use chrono::prelude::{DateTime, Local, Timelike};
use log::trace;
use std::time::{Duration, SystemTime};

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Borders, Paragraph, Widget,
    },
};

use common::models::weather::WeatherInfo;

use crate::utilities::fit_into;

#[derive(Debug)]
pub struct WeatherComponent {
    pub last_refresh: SystemTime,
    pub weather: Result<WeatherInfo, String>,
    pub cooldown: Duration,
}

impl WeatherComponent {
    pub fn new(weather: Result<WeatherInfo, String>) -> WeatherComponent {
        let mut w = WeatherComponent::default();
        w.last_refresh = SystemTime::now();
        w.weather = weather;
        w
    }
}

impl Default for WeatherComponent {
    fn default() -> WeatherComponent {
        WeatherComponent {
            last_refresh: SystemTime::now(),
            weather: Err("No weather was fetched yet".to_string()),
            cooldown: Duration::from_secs(30 * 60),
        }
    }
}

impl Widget for &WeatherComponent {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let last_refreshed = Title::from(Line::from(
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
        ));

        let weather_block = Block::new()
            .borders(Borders::RIGHT)
            .title(
                last_refreshed
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::THICK);

        let counter_text: Text = match &self.weather {
            Ok(weather) => {
                let sunrise = {
                    let timestamp = weather.sys.sunrise;
                    let sunrise = DateTime::from_timestamp(timestamp, 0)
                        .unwrap()
                        .with_timezone(&Local);
                    format!("{:02}:{:02}", sunrise.hour(), sunrise.minute())
                };
                let sunset = {
                    let timestamp = weather.sys.sunset;
                    let sunset = DateTime::from_timestamp(timestamp, 0)
                        .unwrap()
                        .with_timezone(&Local);
                    format!("{:02}:{:02}", sunset.hour(), sunset.minute())
                };

                let separator = "-".repeat((0.66 * area.width as f32) as usize);

                Text::from(vec![
                    Line::from(""),
                    Line::from(vec![
                        format!("{:.0}", weather.main.temp).bold(),
                        "°C".into(),
                    ])
                    .centered(),
                    Line::from(""),
                    Line::from(vec![
                        "\nFeel: ".into(),
                        format!("{:.0}", weather.main.feels_like).yellow(),
                        "°C | ⬇️ ".into(),
                        format!("{:.0}", weather.main.feels_like).yellow(),
                        "°C | ⬆️ ".into(),
                        format!("{:.0}", weather.main.feels_like).yellow(),
                        "°C".into(),
                    ])
                    .centered(),
                    Line::from(separator.clone()).centered(),
                    Line::from("Weather".bold()).centered(),
                    Line::from(""),
                    Line::from(match weather.weather[0].icon.as_str() {
                        "01d" => "☀",   // Sun
                        "01n" => "🌕",  // Moon
                        "02d" => "☀☁",  // Sun with clouds
                        "02n" => "🌕☁", // Moon with clouds
                        "03d" => "☁☁",  // Clouds (day)
                        "03n" => "☁☁",  // Clouds (night)
                        "04d" => "☁☁",  // Menacing clouds (day)
                        "04n" => "☁☁",  // Menacing clouds (night)
                        "09d" => "🌧️",   // Rain (day)
                        "09n" => "🌧️",   // Rain (night)
                        "10d" => "☀🌧️",  // Sun & rain
                        "10n" => "🌕🌧️", // Moon and rain
                        "11d" => "⛈",   // Thunder (day)
                        "11n" => "⛈",   // Thunder (night)
                        "13d" => "🌨️",   // Snowy (day)
                        "13n" => "🌨️",   // Snowy (night)
                        "50d" => "🌫",   // Misty (day)
                        "50n" => "🌫",   // Misty (night)
                        _ => "?",
                    })
                    .centered(),
                    Line::from(format!("{}", weather.weather[0].description).blue()).centered(),
                    Line::from(""),
                    Line::from(separator.clone()).centered(),
                    Line::from("🌕 Day time ☀️".bold()).centered(),
                    Line::from(""),
                    Line::from(format!("🌅 {} 🌄 {}", sunrise, sunset)).centered(),
                ])
            }
            Err(e) => {
                let error_lines = fit_into(e.to_string(), (area.width - 2) as usize);
                let mut lines: Vec<Line> = Vec::new();

                for _ in 1..(area.height - error_lines.len() as u16) / 2 {
                    lines.push(Line::from(""))
                }

                lines.push(Line::from("Error !".red().bold()).centered());
                for line in error_lines {
                    lines.push(Line::from(line.yellow()).centered());
                }

                Text::from(lines)
            }
        };

        Paragraph::new(counter_text)
            .block(weather_block)
            .render(area, buf);
    }
}
