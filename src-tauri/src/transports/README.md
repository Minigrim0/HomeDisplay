# 🚗 Transports API

## 🚚 Models
All the models below are store in the redis database, and refreshed when needed.

### Site
A bus/metro/tram stop.
```rust
pub name: String  // The name of the stop (e.g. T-Centralen)
pub site_id: String  // The ID of the stop, used to fetch new departures from the API
pub coord: Coordinates // The coordinates of the stop, potentially to display the stop on a map
pub departures: Vec<Departure>  // A vector of departure from this stop
```

### Deviation
Deviation information linked to a stop.
```rust
pub importance_level: i32 // How important the deviation is (read SL api doc for a better description)
pub message: String  // Details on the deviation
```

### Departure
Represents a depature for a given line at a given stop.
```rust
pub destination: String  // The destination/direction of the line
pub display: String  // The display time before departure
pub line: i32 // The line number
```


## 📡 API
The api part implements some models, used only to represent the API answers.