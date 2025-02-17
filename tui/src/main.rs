use std::io;
use std::default::Default;
use std::fs::File;

use log;
use clap::Parser;
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};

use common::settings::Settings;

mod app;
mod currency;
mod datetime;
mod transports;
mod tui;
mod utilities;
mod weather;

use app::App;


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author="Minigrim0", version, about="The TUI version of home-display")]
struct Args {
    #[arg(short = 'f', long, default_value = "homedisplay.log")]
    /// File to log to. Defaults to homedisplay.log
    log_file: String,

    #[arg(short = 'l', long, default_value = "WARN")]
    /// Log level, default to WARN. Possible values: TRACE, DEBUG, INFO, WARN, ERROR
    log_level: String,

    #[arg(short = 's', long, default_value = "settings.toml")]
    /// Path to the settings file
    settings: String,

    #[arg(short = 'd', long)]
    /// Dumps the settings file (or a default config if no settings file can be found)
    dump_settings: bool,

    #[arg(short = 'D', long)]
    dump_default_settings: bool,

    #[arg(short = 'o', long, default_value = "settings.toml")]
    /// Output files for the settings dump.
    dump_file: String,
}

fn main() -> io::Result<()> {
    let args: Args = Args::parse();
    let log_file = args.log_file;

    let log_level = match args.log_level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => {
            log::warn!("Unknown log level {}, defaulting to Warn", args.log_level);
            LevelFilter::Warn
        }
    };

    CombinedLogger::init(
        vec![
            WriteLogger::new(log_level, Config::default(), File::create(log_file).unwrap()),
        ]
    ).unwrap();

    if args.dump_settings || args.dump_default_settings {
        let settings = if args.dump_default_settings {
            Settings::default()
        } else {
            Settings::load_from_file(&args.settings).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
        };

        let settings_str = settings.to_string().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        std::fs::write(&args.dump_file, settings_str).unwrap();
        log::info!("Settings dumped to {}", args.dump_file);
        return Ok(());
    }

    let mut terminal = tui::init()?;
    let app_result = App::default()
        .with_settings(&args.settings)
        .run(&mut terminal);
    tui::restore()?;
    app_result
}
