use std::env::var;

pub fn fetch_weather(){
    // https://api.openweathermap.org/data/2.5/weather?lat=59.41709&lon=17.97785&appid=d35ed37117737ce0e9c56f8bb91a02dd

    let api_key = var("OWM_API_KEY").expect("OWM_API_KEY is required to run this hook").to_string();

    let latitude: f32 = var("OWM_LAT").unwrap_or(
        {
            println!("Using default latitude value (Err: Missing OWM_LAT)");
            "59.0".to_string()
        }
    ).parse::<f32>().unwrap_or(
        {
            println!("Could not convert the given latitude value to f32, using default");
            59.0
        }
    );

    let longitude: f32 = var("OWM_LON").unwrap_or(
        {
            println!("Using default latitude value (Err: Missing OWM_LON)");
            "17.0".to_string()
        }
    ).parse::<f32>().unwrap_or(
        {
            println!("Could not convert the given longitude value to f32, using default");
            17.0
        }
    );


    let res = CompanyQuote::get(&symbol, &api_key).await?;
    println!("{}'s current stock price: {}", symbol, res.c);

    Ok(())

}
