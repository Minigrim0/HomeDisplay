use crate::models;

pub fn store_weather(weather: Option<models::weather::WeatherInfo>) -> Result<models::weather::WeatherInfo, &'static str> {
    match weather {
        Some(weather) => {
            // Save the weather in redis
            Ok(weather)
        },
        None => Err("Weather information could not be saved, weather is null.")
    }
}

pub fn store_conversion(conversion: Option<models::currency::Conversion>) -> Result<models::currency::Conversion, &'static str> {
    match conversion {
        Some(conversion) => {
            // Save the weather in redis
            Ok(conversion)
        },
        None => Err("Conversion information could not be saved, conversion is null.")
    }
}

pub fn store_bus_stops(bus_stops: Option<Vec<models::transports::BusStop>>) -> Result<Vec<models::transports::BusStop>, &'static str> {
    match bus_stops {
        Some(bus_stops) => {
            // Save the weather in redis
            Ok(bus_stops)
        },
        None => Err("Bus stops information could not be saved, bus stops are null.")
    }
}
