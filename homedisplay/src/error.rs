use std::fmt;

#[derive(Debug, Clone)]
pub enum HomeDisplayError {
    RedisConnection(String),
    RedisOperation(String),
    ApiRequest(String),
    DataParsing(String),
    SettingsLoad(String),
    SettingsSerialization(String),
    WeatherCodeInvalid(i32),
    DateTimeConversion(String),
    FileOperation(String),
    NetworkTimeout(String),
    InvalidConfiguration(String),
}

impl fmt::Display for HomeDisplayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HomeDisplayError::RedisConnection(msg) => write!(f, "Redis connection failed: {}", msg),
            HomeDisplayError::RedisOperation(msg) => write!(f, "Redis operation failed: {}", msg),
            HomeDisplayError::ApiRequest(msg) => write!(f, "API request failed: {}", msg),
            HomeDisplayError::DataParsing(msg) => write!(f, "Data parsing failed: {}", msg),
            HomeDisplayError::SettingsLoad(msg) => write!(f, "Settings load failed: {}", msg),
            HomeDisplayError::SettingsSerialization(msg) => write!(f, "Settings serialization failed: {}", msg),
            HomeDisplayError::WeatherCodeInvalid(code) => write!(f, "Invalid weather code: {}", code),
            HomeDisplayError::DateTimeConversion(msg) => write!(f, "DateTime conversion failed: {}", msg),
            HomeDisplayError::FileOperation(msg) => write!(f, "File operation failed: {}", msg),
            HomeDisplayError::NetworkTimeout(msg) => write!(f, "Network timeout: {}", msg),
            HomeDisplayError::InvalidConfiguration(msg) => write!(f, "Invalid configuration: {}", msg),
        }
    }
}

impl std::error::Error for HomeDisplayError {}

impl From<std::io::Error> for HomeDisplayError {
    fn from(err: std::io::Error) -> Self {
        HomeDisplayError::FileOperation(err.to_string())
    }
}

impl From<redis::RedisError> for HomeDisplayError {
    fn from(err: redis::RedisError) -> Self {
        HomeDisplayError::RedisOperation(err.to_string())
    }
}

impl From<reqwest::Error> for HomeDisplayError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            HomeDisplayError::NetworkTimeout(err.to_string())
        } else {
            HomeDisplayError::ApiRequest(err.to_string())
        }
    }
}

impl From<serde_json::Error> for HomeDisplayError {
    fn from(err: serde_json::Error) -> Self {
        HomeDisplayError::DataParsing(err.to_string())
    }
}

impl From<toml::de::Error> for HomeDisplayError {
    fn from(err: toml::de::Error) -> Self {
        HomeDisplayError::SettingsLoad(err.to_string())
    }
}

impl From<toml::ser::Error> for HomeDisplayError {
    fn from(err: toml::ser::Error) -> Self {
        HomeDisplayError::SettingsSerialization(err.to_string())
    }
}

pub type HomeDisplayResult<T> = Result<T, HomeDisplayError>;

impl HomeDisplayError {
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            HomeDisplayError::RedisConnection(_) |
            HomeDisplayError::ApiRequest(_) |
            HomeDisplayError::NetworkTimeout(_)
        )
    }
}