# Home Display 🏠

This project aims to provide timings for nearby bus stop, meteo information and currency information

# Organisation 🛠️
There are two main components to this project.

The fetcher binary is launched several times a day to fetch data from the APIs and cache it in a redis database.

The Tauri app is used to display & interact with the results from the APIs (cached in redis). It serves as a desktop application that displays (~in real time)
the data of the different APIs.

# Tauri App 💻
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
export SL_STOPS=<stop_name>,...
```

# Fetcher 🎯
The fetcher is a simple executable that performs the calls to the different APIs and stores the results in the redis database.

It can be run by calling `cargo run --bin fetcher` from the `src-tauri` folder.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Type Support For `.vue` Imports in TS

Since TypeScript cannot handle type information for `.vue` imports, they are shimmed to be a generic Vue component type by default. In most cases this is fine if you don't really care about component prop types outside of templates. However, if you wish to get actual prop types in `.vue` imports (for example to get props validation when using manual `h(...)` calls), you can enable Volar's Take Over mode by following these steps:

1. Run `Extensions: Show Built-in Extensions` from VS Code's command palette, look for `TypeScript and JavaScript Language Features`, then right click and select `Disable (Workspace)`. By default, Take Over mode will enable itself if the default TypeScript extension is disabled.
2. Reload the VS Code window by running `Developer: Reload Window` from the command palette.

You can learn more about Take Over mode [here](https://github.com/johnsoncodehk/volar/discussions/471).


# APIs
* [openweathermap](https://home.openweathermap.org/) For weather information
* [openexchangerate](https://openexchangerates.org/) For currency conversion
* [sl.se](https://sl.se/) (through [trafiklab.se](https://www.trafiklab.se/)) For real-time transports information in Stockholm
