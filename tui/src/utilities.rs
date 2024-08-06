use std::time::SystemTime;

use crate::transports::TransportComponent;
use crate::weather::WeatherComponent;
use crate::currency::CurrencyComponent;

use common::weather::database::fetch_current_weather;
use common::currency::database::fetch_current_conversion;
use common::transports::database::{get_sites, get_departures};


pub fn refresh_weather() -> WeatherComponent {
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build() {
            Ok(rt) => rt,
            Err(e) => {
                let mut weather: WeatherComponent = WeatherComponent::default();
                weather.weather = Err(format!("Unable to build a tokio runtime to fetch the weather {}", e.to_string()));
                return weather
        }
    };

    match rt.block_on(fetch_current_weather()) {
        Ok(weather) => WeatherComponent::new(Ok(weather)),
        Err(e) => WeatherComponent::new(Err(e.to_string()))
    }
}


pub fn refresh_conversion() -> CurrencyComponent {
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build() {
            Ok(rt) => rt,
            Err(e) => {
                let mut conversion: CurrencyComponent = CurrencyComponent::default();
                conversion.conversion = Err(format!("Unable to build a tokio runtime to fetch the currency conversion {}", e.to_string()));
                return conversion
        }
    };

    match rt.block_on(fetch_current_conversion()) {
        Ok(currency) => CurrencyComponent::new(Ok(currency)),
        Err(e) => CurrencyComponent::new(Err(e.to_string()))
    }
}


pub fn refresh_sites(component: &mut TransportComponent) {
    let rt = match tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build() {
            Ok(rt) => rt,
            Err(e) => {
                component.departures.error = Some(format!("Unable to build a tokio runtime to fetch the departures conversion {}", e.to_string()));
                return
        }
    };

    // Reset the error message by default
    component.departures.error = None;

    let sites = match rt.block_on(get_sites()) {
        Ok(currency) => currency,
        Err(e) => {
            component.departures.error = Some(format!("Unable to fetch the sites {}", e.to_string()));
            return
        }
    };

    for site in sites.iter() {
        let departures = match rt.block_on(get_departures(site.id.clone())) {
            Ok(departures) => departures,
            Err(e) => {
                component.departures.site_errors.insert(site.id.clone(), e.to_string());
                continue;
            }
        };

        component.departures.departures.insert(site.id.clone(), departures);
    }

    component.departures.sites = sites;
    component.last_refresh = SystemTime::now();
}
