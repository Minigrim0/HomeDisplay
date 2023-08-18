extern crate redis;
use colored::Colorize;
use redis::Commands;


pub fn get_redis_connection() -> Option<redis::Connection> {
    let client = match redis::Client::open(
        format!("redis://{}:{}/",
            std::env::var("REDIS_HOST")
                .expect("This application needs the REDIS_HOST variable to be set"),
            std::env::var("REDIS_PORT")
                .expect("This application needs the REDIS_PORT variable to be set"),
        ))
    {
            Ok(client) => client,
            Err(_) => {
                println!("{}", "Could not connect to redis".red());
                return None;
            }
    };

    match client.get_connection() {
        Ok(connection) => Some(connection),
        Err(error) => {
            println!("{}", format!("Could not connect to redis: {}", error).red());
            return None
        }
    }
}

pub async fn get_redis_key(key: String) -> Option<String> {
    let mut connection = match get_redis_connection() {
        Some(connection) => connection,
        None => return None
    };

    match connection.get::<String, Option<String>>(key) {
        Err(error) => {
            println!("An error occured while fetching the conversion from redis: {}", error.to_string());
            None
        },
        Ok(None) => {
            println!("No conversion stored in the database");
            None
        },
        Ok(Some(serialized)) => {
            Some(serialized)
        }
    }
}

pub async fn scan_iter(pattern: String) -> Option<Vec<String>> {
    let mut connection = match get_redis_connection() {
        Some(connection) => connection,
        None => return None
    };

    let values: Vec<String>;
    match connection.scan_match(pattern) {
        Ok(iterator) => {
            values = iterator.collect();
            Some(values)
        },
        Err(error) => {
            println!("Unable to get key iterator: {}", error);
            None
        }
    }
}