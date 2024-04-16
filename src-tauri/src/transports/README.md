# Transports module
Manages the departures and sites.

## Database
The sites are stored in the redis database under the key `homedisplay:sites:<site_id>` as a simple json object.
The departures are stored in the redis database under the key `homedisplay:departures:<site_id>` as a list.
