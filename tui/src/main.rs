use std::io;
use std::default::Default;
use std::fs::File;

use log;
use clap::Parser;
use simplelog::{CombinedLogger, Config, LevelFilter, WriteLogger};

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
    #[arg(short = 'f', long)]
    /// File to log to. Defaults to homedisplay.log
    log_file: Option<String>,

    #[arg(short = 'l', long)]
    /// Log level, default to WARN. Possible values: TRACE, DEBUG, INFO, WARN, ERROR
    log_level: Option<String>,
}

fn main() -> io::Result<()> {
    let args: Args = Args::parse();
    let log_file = match &args {
        Args {log_file: Some(filepath), .. } => filepath.clone(),
        _ => "homedisplay.log".to_string()
    };

    let log_level = match &args {
        Args {log_level: Some(level), ..} => match level.to_lowercase().as_str() {
            "trace" => LevelFilter::Trace,
            "debug" => LevelFilter::Debug,
            "info" => LevelFilter::Info,
            "warn" => LevelFilter::Warn,
            "error" => LevelFilter::Error,
            _ => {
                log::warn!("Unknown log level {}, defaulting to Warn", level);
                LevelFilter::Warn
            }
        },
        _ => LevelFilter::Warn
    };

    CombinedLogger::init(
        vec![
            WriteLogger::new(log_level, Config::default(), File::create(log_file).unwrap()),
        ]
    ).unwrap();

    let mut terminal = tui::init()?;
    let app_result = App::default().run(&mut terminal);
    tui::restore()?;
    app_result
}
