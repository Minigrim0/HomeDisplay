# Home Display 🏠

This project aims to provide timings for nearby bus stop, meteo information and currency information. It is intended to run on a controlled environment on a raspberry-pi.

![homedisplay](.github/homedisplay.png)

# Organisation 🛠️
The Tauri app is used to display & interact with the results from the APIs (cached in redis). It serves as a desktop application that displays (~in real time)
the data of the different APIs. It relies on an external redis storage to cache results.

# App 💻
To run the tauri application run `npm run tauri dev`. This will build the app and start it in development mode.
In order to be able to use the APIs directly, some variables need to be set.
```bash
export OWM_API_KEY=<Your api key for openweathermap>
export OER_API_KEY=<Your api key for openexchangerate>
export SL_PLACE_API_KEY=<Your api key for SL Platsuppslag>
export SL_PLACE_ROOT_URL=<The base url for the places API>
export SL_PLACE_BUS_STOPS=<The stops to monitor, separated by a comma>
export SL_REALTIME_API_KEY=<Your api key for SL Realtidsinformation 4>
export SL_REALTIME_ROOT_URL=<The base url for the realtid API>
```

To set the latitude and longitude of the location to get the weather information for, you will need to export the following variables:
```bash
export OWM_LAT=<latitude of the point>
export OWM_LON=<longitude of the point>
```

To change the currency conversions
```bash
export OER_FROM=<Base currency code>  # e.g. EUR
export OER_TO=<Currency to convert to>  # e.g. SEK
```

To set the stop(s) to watch time for
```bash
export SL_PLACE_BUS_STOPS=<stop_name>,...
```

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)


## Arm compilation
This project is intended to run on a raspberryPi. The script `arm_comp.sh` serve to compile and build a `.deb` package for raspbian. Only ubuntu is supported as of now.

# APIs
* [openweathermap](https://home.openweathermap.org/) For weather information
* [openexchangerate](https://openexchangerates.org/) For currency conversion
* [sl.se](https://sl.se/) (through [trafiklab.se](https://www.trafiklab.se/)) For real-time transports information in Stockholm
