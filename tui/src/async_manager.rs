/// Async data manager for handling background data fetching
///
/// This module provides a single Tokio runtime with background tasks that fetch
/// weather, currency, and transport data concurrently. It bridges async operations
/// with the synchronous TUI event loop using channels.

use std::sync::mpsc;
use std::time::Duration;
use tokio::sync::oneshot;
use log::{info, error};

use homedisplay::settings::{Settings, Weather, Currency, BusStop, Redis};
use homedisplay::models::{
    weather::WeatherInfo,
    currency::Conversion,
    transports::{Site, Departure},
};

use crate::error::{TuiError, TuiResult};

/// Data updates sent from async tasks to the UI thread
#[derive(Debug, Clone)]
pub enum DataUpdate {
    Weather(Result<WeatherInfo, TuiError>),
    Currency(Result<Conversion, TuiError>),
    Transport(TransportUpdate),
}

/// Transport-specific update containing sites and their departures
#[derive(Debug, Clone)]
pub struct TransportUpdate {
    pub sites: Vec<Site>,
    pub departures: std::collections::HashMap<String, Vec<Departure>>,
    pub site_errors: std::collections::HashMap<String, TuiError>,
    pub error: Option<TuiError>,
}

/// Configuration for data refresh intervals
#[derive(Debug, Clone)]
pub struct RefreshConfig {
    pub weather_interval: Duration,
    pub currency_interval: Duration,
    pub transport_interval: Duration,
}

impl Default for RefreshConfig {
    fn default() -> Self {
        Self {
            weather_interval: Duration::from_secs(30 * 60), // 30 minutes
            currency_interval: Duration::from_secs(60 * 60), // 60 minutes
            transport_interval: Duration::from_secs(60),     // 1 minute
        }
    }
}

/// Async data manager that runs background tasks and communicates with UI thread
pub struct AsyncDataManager {
    runtime: tokio::runtime::Runtime,
    shutdown_tx: Option<oneshot::Sender<()>>,
}

impl AsyncDataManager {
    /// Creates a new async data manager with a single Tokio runtime
    pub fn new() -> TuiResult<Self> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|e| TuiError::TokioRuntime(format!("Failed to create runtime: {}", e)))?;

        Ok(Self {
            runtime,
            shutdown_tx: None,
        })
    }

    /// Starts background data fetching tasks and returns a receiver for updates
    ///
    /// This method spawns three background tasks that continuously fetch data
    /// and send updates through the returned channel.
    pub fn start_background_tasks(
        &mut self,
        settings: Settings,
        config: RefreshConfig,
    ) -> TuiResult<mpsc::Receiver<DataUpdate>> {
        info!("Starting async data manager with intervals: weather={}s, currency={}s, transport={}s",
              config.weather_interval.as_secs(), config.currency_interval.as_secs(), config.transport_interval.as_secs());

        let (tx, rx) = mpsc::channel();
        let (shutdown_tx, shutdown_rx) = oneshot::channel();

        self.shutdown_tx = Some(shutdown_tx);

        // Clone settings for each task
        let weather_settings = settings.weather.clone();
        let currency_settings = settings.currency.clone();
        let transport_settings = settings.transports.clone();
        let redis_settings = settings.redis.clone();

        // Spawn weather task
        info!("Spawning weather background task");
        let weather_tx = tx.clone();
        let weather_redis = redis_settings.clone();
        self.runtime.spawn(async move {
            Self::weather_task(weather_settings, weather_redis, weather_tx, config.weather_interval).await;
        });

        // Spawn currency task
        info!("Spawning currency background task");
        let currency_tx = tx.clone();
        let currency_redis = redis_settings.clone();
        self.runtime.spawn(async move {
            Self::currency_task(currency_settings, currency_redis, currency_tx, config.currency_interval).await;
        });

        // Spawn transport task
        info!("Spawning transport background task");
        let transport_tx = tx.clone();
        let transport_redis = redis_settings;
        self.runtime.spawn(async move {
            Self::transport_task(transport_settings, transport_redis, transport_tx, config.transport_interval).await;
        });

        // Spawn shutdown monitoring task
        info!("Spawning shutdown monitoring task");
        self.runtime.spawn(async move {
            let _ = shutdown_rx.await;
            info!("Shutdown signal received, async data manager shutting down");
        });

        info!("Background data tasks started");
        Ok(rx)
    }

    /// Background task for fetching weather data
    async fn weather_task(
        settings: Weather,
        redis: Redis,
        tx: mpsc::Sender<DataUpdate>,
        interval: Duration,
    ) {
        info!("Weather task started with interval: {}s", interval.as_secs());
        let mut interval_timer = tokio::time::interval(interval);

        loop {
            info!("Weather task: Starting data fetch");
            let result = match homedisplay::weather::database::fetch_current_weather(settings.clone(), &redis).await {
                Ok(weather) => {
                    info!("Weather task: Data fetched successfully - temp: {:.1}°C", weather.current.temperature_2m);
                    Ok(weather)
                },
                Err(e) => {
                    error!("Weather task: Failed to fetch data: {}", e);
                    Err(TuiError::WeatherFetch(e))
                }
            };

            if let Err(e) = tx.send(DataUpdate::Weather(result)) {
                error!("Weather task: Failed to send update to UI thread: {}", e);
                break;
            } else {
                info!("Weather task: Update sent to UI thread successfully");
            }

            interval_timer.tick().await;
        }
    }

    /// Background task for fetching currency data
    async fn currency_task(
        settings: Currency,
        redis: Redis,
        tx: mpsc::Sender<DataUpdate>,
        interval: Duration,
    ) {
        info!("Currency task started with interval: {}s", interval.as_secs());
        let mut interval_timer = tokio::time::interval(interval);

        loop {
            info!("Currency task: Starting data fetch");
            let result = match homedisplay::currency::database::fetch_current_conversion(settings.clone(), &redis).await {
                Ok(currency) => {
                    info!("Currency task: Data fetched successfully - {} {} = {} {}",
                          currency.from_currency_amount, currency.from_currency,
                          currency.to_currency_amount, currency.to_currency);
                    Ok(currency)
                },
                Err(e) => {
                    error!("Currency task: Failed to fetch data: {}", e);
                    Err(TuiError::CurrencyFetch(e))
                }
            };

            if let Err(e) = tx.send(DataUpdate::Currency(result)) {
                error!("Currency task: Failed to send update to UI thread: {}", e);
                break;
            } else {
                info!("Currency task: Update sent to UI thread successfully");
            }

            interval_timer.tick().await;
        }
    }

    /// Background task for fetching transport data
    async fn transport_task(
        stops: Vec<BusStop>,
        redis: Redis,
        tx: mpsc::Sender<DataUpdate>,
        interval: Duration,
    ) {
        info!("Transport task started with interval: {}s, monitoring {} stops", interval.as_secs(), stops.len());
        let mut interval_timer = tokio::time::interval(interval);

        loop {
            info!("Transport task: Starting data fetch for {} stops", stops.len());
            let mut transport_update = TransportUpdate {
                sites: Vec::new(),
                departures: std::collections::HashMap::new(),
                site_errors: std::collections::HashMap::new(),
                error: None,
            };

            // Fetch sites
            let sites = match homedisplay::transports::database::get_sites(&stops, &redis).await {
                Ok(sites) => {
                    info!("Transport task: Fetched {} sites successfully", sites.len());
                    sites
                },
                Err(e) => {
                    error!("Transport task: Failed to fetch sites: {}", e);
                    transport_update.error = Some(TuiError::TransportFetch(format!("Failed to fetch sites: {}", e)));
                    if let Err(send_err) = tx.send(DataUpdate::Transport(transport_update)) {
                        error!("Transport task: Failed to send error update to UI thread: {}", send_err);
                    }
                    continue;
                }
            };

            // Fetch departures for each site
            info!("Transport task: Fetching departures for {} sites", sites.len());
            for site in &sites {
                info!("Transport task: Fetching departures for site {} ({})", site.id, site.name);
                let departures = match homedisplay::transports::database::get_departures(site.id.clone(), &redis).await {
                    Ok(departures) => {
                        // Apply line filtering if configured
                        let filtered_departures = if let Some(stop) = stops.iter().find(|s| s.name == site.name) {
                            if let Some(preferred_lines) = &stop.preffered_lines {
                                departures.into_iter()
                                    .filter(|d| preferred_lines.contains(&d.line.id))
                                    .collect()
                            } else {
                                departures
                            }
                        } else {
                            departures
                        };

                        info!("Transport task: Site {} - {} departures after filtering", site.id, filtered_departures.len());
                        filtered_departures
                    },
                    Err(e) => {
                        error!("Transport task: Failed to fetch departures for site {} ({}): {}", site.id, site.name, e);
                        transport_update.site_errors.insert(
                            site.id.clone(),
                            TuiError::TransportFetch(format!("Failed to fetch departures: {}", e))
                        );
                        continue;
                    }
                };

                transport_update.departures.insert(site.id.clone(), departures);
            }

            // Filter out sites with no departures
            let original_count = sites.len();
            transport_update.sites = sites.into_iter()
                .filter(|site| transport_update.departures.get(&site.id).map_or(false, |deps| !deps.is_empty()))
                .collect();

            info!("Transport task: Data processed - {} sites with departures (filtered from {})",
                  transport_update.sites.len(), original_count);

            if let Err(e) = tx.send(DataUpdate::Transport(transport_update)) {
                error!("Transport task: Failed to send update to UI thread: {}", e);
                break;
            } else {
                info!("Transport task: Update sent to UI thread successfully");
            }

            interval_timer.tick().await;
        }
    }

    /// Shuts down the async data manager and all background tasks
    pub fn shutdown(mut self) {
        if let Some(shutdown_tx) = self.shutdown_tx.take() {
            let _ = shutdown_tx.send(());
        }

        self.runtime.shutdown_timeout(Duration::from_secs(5));
        info!("Async data manager shut down");
    }
}

impl std::fmt::Debug for AsyncDataManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AsyncDataManager")
            .field("shutdown_tx", &self.shutdown_tx.is_some())
            .finish_non_exhaustive()
    }
}
