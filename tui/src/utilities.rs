use log::trace;
use std::time::SystemTime;

use crate::currency::CurrencyComponent;
use crate::transports::TransportComponent;
use crate::weather::WeatherComponent;

use common::currency::database::fetch_current_conversion;
use common::transports::database::{get_departures, get_sites};
use common::weather::database::fetch_current_weather;
use common::settings;

pub fn refresh_weather(weather_settings: settings::Weather, redis_data: &settings::Redis) -> WeatherComponent {
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => {
            let mut weather: WeatherComponent = WeatherComponent::default();
            weather.weather = Err(format!(
                "Unable to build a tokio runtime to fetch the weather {}",
                e.to_string()
            ));
            return weather;
        }
    };

    match rt.block_on(fetch_current_weather(weather_settings, redis_data)) {
        Ok(weather) => WeatherComponent::new(Ok(weather)),
        Err(e) => WeatherComponent::new(Err(e.to_string())),
    }
}

pub fn refresh_conversion(currency_settings: settings::Currency, redis_data: &settings::Redis) -> CurrencyComponent {
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => {
            let mut conversion: CurrencyComponent = CurrencyComponent::default();
            conversion.conversion = Err(format!(
                "Unable to build a tokio runtime to fetch the currency conversion {}",
                e.to_string()
            ));
            return conversion;
        }
    };

    match rt.block_on(fetch_current_conversion(currency_settings, redis_data)) {
        Ok(currency) => CurrencyComponent::new(Ok(currency)),
        Err(e) => CurrencyComponent::new(Err(e.to_string())),
    }
}

pub fn refresh_sites(component: &mut TransportComponent, stops: Vec<settings::BusStop>, redis_data: &settings::Redis) {
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => {
            component.departures.error = Some(format!(
                "Unable to build a tokio runtime to fetch the departures conversion {}",
                e.to_string()
            ));
            return;
        }
    };

    // Reset the error message by default
    component.departures.error = None;
    component.departures.site_errors.clear();

    let sites = match rt.block_on(get_sites(stops, redis_data)) {
        Ok(currency) => currency,
        Err(e) => {
            component.departures.error =
                Some(format!("Unable to fetch the sites {}", e.to_string()));
            return;
        }
    };

    for site in sites.iter() {
        let departures = match rt.block_on(get_departures(site.id.clone(), redis_data)) {
            Ok(departures) => departures,
            Err(e) => {
                component
                    .departures
                    .site_errors
                    .insert(site.id.clone(), e.to_string());
                continue;
            }
        };

        component
            .departures
            .departures
            .insert(site.id.clone(), departures);
    }

    component.departures.sites = sites;
    component.last_refresh = SystemTime::now();
}

/// Splits the content by spaces, and re-arranges it into an iterator
/// where each element fits the size.
pub fn fit_into(content: String, size: usize) -> Vec<String> {
    let splitted = content.split(" ");
    let mut lines: Vec<String> = Vec::new();

    let mut current_string_part: String = String::new();
    for element in splitted {
        trace!("Working on word: {}", &element);
        if format!("{} {}", current_string_part, element).len() < size {
            current_string_part = format!("{} {}", current_string_part, element);
            trace!("Expanding current_string: {}", &current_string_part);
        } else {
            lines.push(current_string_part);
            current_string_part = element.to_string();
        }
    }
    // Final push
    lines.push(current_string_part);

    lines
}
