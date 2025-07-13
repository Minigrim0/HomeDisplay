use log::{trace, info};
use std::time::SystemTime;

use crate::currency::CurrencyComponent;
use crate::error::TuiError;
use crate::transports::TransportComponent;
use crate::weather::WeatherComponent;

use homedisplay::currency::database::fetch_current_conversion;
use homedisplay::transports::database::{get_departures, get_sites};
use homedisplay::models::transports::Departure;
use homedisplay::weather::database::fetch_current_weather;
use homedisplay::settings;

/// Refreshes weather data by creating a tokio runtime and fetching from the database
pub fn refresh_weather(weather_settings: settings::Weather, redis_data: &settings::Redis) -> WeatherComponent {
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => {
            let mut weather: WeatherComponent = WeatherComponent::default();
            weather.weather = Err(TuiError::TokioRuntime(
                format!("Unable to build tokio runtime for weather: {}", e)
            ));
            return weather;
        }
    };

    match rt.block_on(fetch_current_weather(weather_settings, redis_data)) {
        Ok(weather) => WeatherComponent::new(Ok(weather)),
        Err(e) => WeatherComponent::new(Err(TuiError::WeatherFetch(e))),
    }
}

/// Refreshes currency conversion data by creating a tokio runtime and fetching from the database
pub fn refresh_conversion(currency_settings: settings::Currency, redis_data: &settings::Redis) -> CurrencyComponent {
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => {
            let mut conversion: CurrencyComponent = CurrencyComponent::default();
            conversion.conversion = Err(TuiError::TokioRuntime(
                format!("Unable to build tokio runtime for currency: {}", e)
            ));
            return conversion;
        }
    };

    match rt.block_on(fetch_current_conversion(currency_settings, redis_data)) {
        Ok(currency) => CurrencyComponent::new(Ok(currency)),
        Err(e) => CurrencyComponent::new(Err(TuiError::CurrencyFetch(e))),
    }
}

/// Refreshes transport departure sites by creating a tokio runtime and fetching from the database
pub fn refresh_sites(component: &mut TransportComponent, stops: Vec<settings::BusStop>, redis_data: &settings::Redis) {
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => {
            component.departures.error = Some(TuiError::TokioRuntime(
                format!("Unable to build tokio runtime for departures: {}", e)
            ));
            return;
        }
    };

    // Reset the error message by default
    component.departures.error = None;
    component.departures.site_errors.clear();

    let sites = match rt.block_on(get_sites(&stops, redis_data)) {
        Ok(site) => site,
        Err(e) => {
            component.departures.error = Some(TuiError::TransportFetch(
                format!("Unable to fetch sites: {}", e)
            ));
            return;
        }
    };

    let mut empty_sites = vec![];

    for site in sites.iter() {
        let mut filter_on = vec![];
        info!("Refreshing site {} ({})", site.id, site.name);
        if let Some(stop_index) = stops.iter().position(|stop| stop.name == site.name) {
            info!("Finding out whether to filter lines for stop: {}", &stops[stop_index].name);
            if let Some(lines) = &stops[stop_index].preffered_lines {
                filter_on = lines.iter().map(|el| *el).collect();
                info!("Filtering stop {} on lines {:?}", stops[stop_index].name, filter_on);
            }
        }

        let departures: Vec<Departure> = match rt.block_on(get_departures(site.id.clone(), redis_data)) {
            Ok(departures) => departures.into_iter().filter(
                |s| (filter_on.is_empty() || filter_on.contains(&s.line.id))
            ).collect(),
            Err(e) => {
                component
                    .departures
                    .site_errors
                    .insert(site.id.clone(), TuiError::TransportFetch(
                        format!("Unable to fetch departures for site {}: {}", site.id, e)
                    ));
                continue;
            }
        };

        if departures.is_empty() {
            empty_sites.push(site.id.clone());
        }

        component
            .departures
            .departures
            .insert(site.id.clone(), departures);
    }

    component.departures.sites = sites.into_iter().filter(|s| !empty_sites.contains(&s.id)).collect();
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
