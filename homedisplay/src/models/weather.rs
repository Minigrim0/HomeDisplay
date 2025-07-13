use serde::{Deserialize, Serialize};
use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeatherInfo {
    pub latitude: f32,
    pub longitude: f32,
    pub current: CurrentWeather,
    pub hourly: HourlyWeather,
    pub daily: DailyWeather,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrentWeather {
    pub time: String,
    pub temperature_2m: f32,
    pub relative_humidity_2m: f32,
    pub apparent_temperature: f32,
    pub rain: f32,
    pub weather_code: i32,
    pub surface_pressure: f32,
    pub wind_speed_10m: f32,
    pub wind_direction_10m: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HourlyWeather {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f32>,
    pub apparent_temperature: Vec<f32>,
    pub precipitation: Vec<f32>,
    pub rain: Vec<f32>,
    pub snowfall: Vec<f32>,
    pub wind_speed_10m: Vec<f32>,
    pub wind_direction_10m: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DailyWeather {
    pub time: Vec<String>,
    pub weather_code: Vec<i32>,
    pub temperature_2m_max: Vec<f32>,
    pub temperature_2m_min: Vec<f32>,
    pub apparent_temperature_max: Vec<f32>,
    pub apparent_temperature_min: Vec<f32>,
    pub sunrise: Vec<String>,
    pub sunset: Vec<String>,
    pub daylight_duration: Vec<f32>,
    pub uv_index_max: Vec<f32>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WeatherForecast {
    pub time: NaiveDate,
    pub weather_code: String,
    pub temperature_2m_max: f32,
    pub temperature_2m_min: f32,
    pub apparent_temperature_max: f32,
    pub apparent_temperature_min: f32,
    pub uv_index_max: f32
}

impl DailyWeather {
    /// Returns wether it is currently day, given the sunset information
    fn is_day(&self) -> bool {
        if let Some(sunset) = self.sunset.first() {
            DateTime::parse_from_rfc3339(sunset).and_then(|dt| Ok(Local::now().fixed_offset().timestamp() - dt.timestamp() < 0)).unwrap_or(true)
        } else {
            false
        }
    }

    fn get_weather_code(&self, code: i32) -> Result<(String, String), String> {
        let codes: super::weather_codes::WeatherCode = serde_json::from_str(super::weather_codes::WEATHER_CODES).map_err(|e| e.to_string())?;

        let info = codes.get(code.to_string().as_str()).and_then(|m|
            if self.is_day() {
                m.get("day")
            } else {
                m.get("night")
            }
        ).ok_or("Unable to get weather info")?;
        let image = info.get("image").ok_or("Unable to get image for weather info")?;
        let description = info.get("description").ok_or("Unable to get description for weather info")?;

        Ok((image.clone(), description.clone()))
    }

    /// Returns a tuple with the weather code and description from the OpenWeatherMap weather code table
    /// https://openweathermap.org/weather-conditions
    pub fn get_weather_info(&self) -> Result<(String, String), String> {
        if let Some(code) = self.weather_code.first() {
            self.get_weather_code(*code)
        } else {
            Err("No weather info could be found".to_string())
        }
    }

    pub fn get_sun_info(&self) -> Result<(DateTime<FixedOffset>, DateTime<FixedOffset>, f32), String> {
        let local_timezone = Local::now().timezone();

        let sunrise = if let Some(sunrise) = self.sunrise.first() {
            NaiveDateTime::parse_from_str(sunrise, "%Y-%m-%dT%H:%M").map_err(|e| {
                format!("Unable to parse sunrise from `{}`: {}", sunrise, e)
            })?
                .and_local_timezone(local_timezone)
                .unwrap()
                .fixed_offset()
        } else {
            Local::now().fixed_offset()
        };

        let sunset = if let Some(sunset) = self.sunset.first() {
            NaiveDateTime::parse_from_str(sunset, "%Y-%m-%dT%H:%M").map_err(|e| {
                format!("Unable to parse sunset from `{}`: {}", sunrise, e)
            })?
                .and_local_timezone(local_timezone)
                .unwrap()
                .fixed_offset()
        } else {
            Local::now().fixed_offset()
        };

        let duration = self.daylight_duration.first().unwrap_or(&0.0);

        Ok((sunrise, sunset, *duration))
    }

    pub fn get_forecast(&self) -> Result<Vec<WeatherForecast>, String> {
        let forecast = self.time.iter()
            .zip(self.weather_code.iter())
            .zip(self.temperature_2m_max.iter())
            .zip(self.temperature_2m_min.iter())
            .zip(self.apparent_temperature_max.iter())
            .zip(self.apparent_temperature_min.iter())
            .zip(self.uv_index_max.iter())
            .map(
                |((((((time, weather_code), temperature_2m_max), temperature_2m_min), apparent_temperature_max), apparent_temperature_min), uv_index_max)|
                {
                let time = NaiveDate::parse_from_str(time, "%Y-%m-%d").map_err(|e| {
                    format!("Unable to parse time from `{}`: {}", time, e)
                })?;
                Ok(WeatherForecast {
                    time,
                    weather_code: self.get_weather_code(*weather_code)?.0,
                    temperature_2m_max: *temperature_2m_max,
                    temperature_2m_min: *temperature_2m_min,
                    apparent_temperature_max: *apparent_temperature_max,
                    apparent_temperature_min: *apparent_temperature_min,
                    uv_index_max: *uv_index_max
                })
            })
            .collect::<Vec<Result<WeatherForecast, String>>>();

        let errors = forecast.iter().filter_map(|r| r.clone().err()).collect::<Vec<String>>();
        if errors.len() > 0 {
            Err(errors.join("\n"))
        } else {
            Ok(forecast.into_iter().filter_map(|r| r.ok()).collect())
        }
    }
}
