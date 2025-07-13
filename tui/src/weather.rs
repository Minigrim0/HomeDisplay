use chrono::prelude::{Local, Timelike};
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

use homedisplay::models::weather::WeatherInfo;

use crate::error::TuiError;
use crate::utilities::fit_into;

#[derive(Debug)]
/// Weather display component that shows current conditions and forecasts
pub struct WeatherComponent {
    pub last_refresh: SystemTime,              // Last time weather data was refreshed
    pub weather: Result<WeatherInfo, TuiError>, // Current weather data or error
    pub cooldown: Duration,                     // Time between refresh attempts
    pub current_forecast_day: u8,               // Currently displayed forecast day (0-6)
    pub last_forecast_change: SystemTime,       // Last time forecast display rotated
}

impl WeatherComponent {
    /// Creates a new weather component with the given weather data
    pub fn new(weather: Result<WeatherInfo, TuiError>) -> WeatherComponent {
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
            weather: Err(TuiError::WeatherFetch("No weather was fetched yet".to_string())),
            cooldown: Duration::from_secs(30 * 60),
            current_forecast_day: 0,
            last_forecast_change: SystemTime::now(),
        }
    }
}

impl Widget for &WeatherComponent {
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

        let weather_block = Block::new()
            .borders(Borders::RIGHT)
            .title_bottom(last_refreshed.centered())
            .border_set(border::THICK);

        let counter_text: Text = match &self.weather {
            Ok(weather) => {
                let mut errors = vec![];

                let sun_info = weather.daily.get_sun_info().map_err(
                    |e| errors.push(e.to_string())
                ).unwrap_or((Local::now().fixed_offset(), Local::now().fixed_offset(), 0.0));

                let sunrise = {
                    format!("{:02}:{:02}",  sun_info.0.hour(),  sun_info.0.minute())
                };
                let sunset = {
                    format!("{:02}:{:02}",  sun_info.1.hour(),  sun_info.1.minute())
                };
                let daytime = {
                    let daytime = sun_info.2 as i32;
                    let hours = daytime / 3600;
                    let minutes = (daytime - hours * 3600) / 60;
                    let seconds = daytime % 60;
                    format!("{:02}h {:02}m {:02}s", hours, minutes, seconds)
                };

                let separator = "-".repeat((0.66 * area.width as f32) as usize);

                let weather_info = weather.daily.get_weather_info().map_err(
                    |e| errors.push(e.to_string())
                ).unwrap_or(("01d".to_string(), "error".to_string()));

                let forecast = match weather.daily.get_forecast() {
                    Ok(f) => f,
                    Err(e) => {
                        errors.push(format!("Unable to get forecast: {}", e.to_string()));
                        vec![]
                    }
                };

                Text::from(vec![
                    Line::from(""),
                    Line::from(vec![
                        format!("{:.0}", weather.current.temperature_2m).bold(),
                        "°C".into(),
                    ])
                    .centered(),
                    Line::from(""),
                    Line::from(vec![
                        "\nFeel: ".into(),
                        format!("{:.0}", weather.current.apparent_temperature).yellow(),
                        "°C | ⬇️ ".into(),
                        format!("{:.0}", weather.daily.apparent_temperature_min.first().unwrap_or(&-1000.0)).yellow(),
                        "°C | ⬆️ ".into(),
                        format!("{:.0}", weather.daily.apparent_temperature_max.first().unwrap_or(&1000.0)).yellow(),
                        "°C".into(),
                    ])
                    .centered(),
                    Line::from(separator.clone()).centered(),
                    Line::from("Weather".bold()).centered().underlined(),
                    Line::from(""),
                    Line::from(match weather_info.0.as_str() {
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
                    Line::from(format!("{}", weather_info.1).blue()).centered(),
                    Line::from(""),
                    Line::from(separator.clone()).centered(),
                    Line::from("Forecast".bold()).centered(),
                    Line::from(format!("{:6} Min | Max |  UV  | F Min | F Max", "Date")).centered(),
                    Line::from("-----------|-----|------|-------|------").centered(),
                    Line::from(match forecast.get(self.current_forecast_day as usize) {
                        Some(f) => format!(
                            "{:6} {:3.0} | {:3.0} | {:1.2} |  {:3.0}  |  {:3.0}",
                            f.time.format("%a %d"),
                            f.temperature_2m_min,
                            f.temperature_2m_max,
                            f.uv_index_max,
                            f.apparent_temperature_min,
                            f.apparent_temperature_max
                        ),
                        None => "No forecast available".to_string(),
                    }).centered(),
                    Line::from(match forecast.get((self.current_forecast_day as usize + 1) % 7) {
                        Some(f) => format!(
                            "{:6} {:3.0} | {:3.0} | {:1.2} |  {:3.0}  |  {:3.0}",
                            f.time.format("%a %d"),
                            f.temperature_2m_min,
                            f.temperature_2m_max,
                            f.uv_index_max,
                            f.apparent_temperature_min,
                            f.apparent_temperature_max
                        ),
                        None => "No forecast available".to_string(),
                    }).centered(),
                    Line::from(match forecast.get((self.current_forecast_day as usize + 2) % 7) {
                        Some(f) => format!(
                            "{:6} {:3.0} | {:3.0} | {:1.2} |  {:3.0}  |  {:3.0}",
                            f.time.format("%a %d"),
                            f.temperature_2m_min,
                            f.temperature_2m_max,
                            f.uv_index_max,
                            f.apparent_temperature_min,
                            f.apparent_temperature_max
                        ),
                        None => "No forecast available".to_string(),
                    }).centered(),
                    Line::from(separator.clone()).centered(),
                    Line::from("🌕 Day time ☀️".bold()).centered(),
                    Line::from(""),
                    Line::from(format!("🌅 {} 🌄 {}", sunrise, sunset)).centered(),
                    Line::from(format!("({})", daytime)).centered(),
                ])
            }
            Err(e) => {
                let user_message = e.user_message();
                let detailed_message = if log::log_enabled!(log::Level::Debug) {
                    e.to_string()
                } else {
                    user_message.to_string()
                };
                
                let error_lines = fit_into(detailed_message, (area.width - 2) as usize);
                let mut lines: Vec<Line> = Vec::new();

                for _ in 1..(area.height - error_lines.len() as u16) / 2 {
                    lines.push(Line::from(""))
                }

                lines.push(Line::from("Weather Error".red().bold()).centered());
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
