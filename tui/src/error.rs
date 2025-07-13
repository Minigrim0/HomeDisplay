use std::fmt;

#[derive(Debug, Clone)]
pub enum TuiError {
    WeatherFetch(String),
    CurrencyFetch(String),
    TransportFetch(String),
    TokioRuntime(String),
    TerminalTooSmall { width: u16, height: u16 },
    TerminalInit(String),
    SettingsLoad(String),
    TimezoneInvalid(String),
    SystemTime(String),
    Io(String),
}

impl fmt::Display for TuiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TuiError::WeatherFetch(msg) => write!(f, "Weather fetch failed: {}", msg),
            TuiError::CurrencyFetch(msg) => write!(f, "Currency fetch failed: {}", msg),
            TuiError::TransportFetch(msg) => write!(f, "Transport fetch failed: {}", msg),
            TuiError::TokioRuntime(msg) => write!(f, "Runtime error: {}", msg),
            TuiError::TerminalTooSmall { width, height } => {
                write!(f, "Terminal too small: {}x{} (minimum 30x5)", width, height)
            }
            TuiError::TerminalInit(msg) => write!(f, "Terminal error: {}", msg),
            TuiError::SettingsLoad(msg) => write!(f, "Settings error: {}", msg),
            TuiError::TimezoneInvalid(msg) => write!(f, "Timezone error: {}", msg),
            TuiError::SystemTime(msg) => write!(f, "System time error: {}", msg),
            TuiError::Io(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for TuiError {}

impl From<std::io::Error> for TuiError {
    fn from(err: std::io::Error) -> Self {
        TuiError::Io(err.to_string())
    }
}

impl From<std::time::SystemTimeError> for TuiError {
    fn from(err: std::time::SystemTimeError) -> Self {
        TuiError::SystemTime(err.to_string())
    }
}

pub type TuiResult<T> = Result<T, TuiError>;

impl TuiError {
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            TuiError::WeatherFetch(_)
                | TuiError::CurrencyFetch(_)
                | TuiError::TransportFetch(_)
                | TuiError::TerminalTooSmall { .. }
        )
    }

    pub fn user_message(&self) -> &str {
        match self {
            TuiError::WeatherFetch(_) => "Weather data unavailable",
            TuiError::CurrencyFetch(_) => "Currency data unavailable",
            TuiError::TransportFetch(_) => "Transport data unavailable",
            TuiError::TokioRuntime(_) => "System error",
            TuiError::TerminalTooSmall { .. } => "Terminal too small",
            TuiError::TerminalInit(_) => "Display error",
            TuiError::SettingsLoad(_) => "Configuration error",
            TuiError::TimezoneInvalid(_) => "Timezone error",
            TuiError::SystemTime(_) => "Time error",
            TuiError::Io(_) => "System error",
        }
    }
}
