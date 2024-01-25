extern crate redis;
use redis::Commands;


pub fn get_redis_connection() -> Result<redis::Connection, String> {
    let client = match redis::Client::open(
        format!("redis://{}:{}/",
            std::env::var("REDIS_HOST").expect_err("This application needs the REDIS_HOST variable to be set"),
            std::env::var("REDIS_PORT").expect_err("This application needs the REDIS_PORT variable to be set"),
        ))
    {
        Ok(client) => client,
        Err(_) => {
            return Err("Could not connect to redis".to_string());
        }
    };

    match client.get_connection() {
        Ok(connection) => Ok(connection),
        Err(error) => Err(format!("Could not connect to redis: {}", error))
    }
}

pub async fn get_redis_key(key: String) -> Result<String, String> {
    let mut connection = get_redis_connection()?;

    match connection.get::<String, Option<String>>(key) {
        Err(error) => {
            Err(format!("An error occured while fetching the conversion from redis: {}", error.to_string()))
        },
        Ok(None) => {
            Err("No conversion stored in the database".to_string())
        },
        Ok(Some(serialized)) => {
            Ok(serialized)
        }
    }
}

pub async fn scan_iter(pattern: String) -> Result<Vec<String>, String> {
    let mut connection = get_redis_connection()?;

    let values: Vec<String>;
    match connection.scan_match(pattern) {
        Ok(iterator) => {
            values = iterator.collect();
            Ok(values)
        },
        Err(error) => Err(format!("Unable to get key iterator: {}", error))
    }
}
