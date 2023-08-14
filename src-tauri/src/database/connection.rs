extern crate redis;
use colored::Colorize;


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