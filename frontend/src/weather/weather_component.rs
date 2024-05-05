use yew::{html, Component, Context, Html};
use chrono::prelude::{Local, DateTime, Timelike};


use common::models::weather::WeatherInfo;

use super::services::start_weather_job;

pub struct WeatherComponent {
    weather: Option<WeatherInfo>,
    loading: bool,
    error: Option<String>,
    last_update: i64,
    time_since_last_update: i64
}

pub enum Msg {
    ClockUpdate,
    LoadWeatherData,
    WeatherDataReceived(Result<WeatherInfo, String>)
}


impl Component for WeatherComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::LoadWeatherData);

        let weather_ready_cb = ctx.link().callback(Msg::WeatherDataReceived);
        start_weather_job(weather_ready_cb);

        Self {
            weather: None,
            loading: false,
            error: None,
            last_update: Local::now().timestamp(),
            time_since_last_update: 0
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClockUpdate => {
                let now = Local::now().timestamp();
                self.time_since_last_update = now - self.last_update;
                true
            }
            Msg::LoadWeatherData => {
                self.loading = true;
                self.error = None;
                self.weather = None;
                self.last_update = Local::now().timestamp(); 
                true
            },
            Msg::WeatherDataReceived(result) => {
                match result {
                    Ok(value) => {
                        self.error = None;
                        self.weather = Some(value);
                    },
                    Err(e) => {
                        self.error = Some(e);
                    }
                }
                self.loading = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(error) = &self.error {
            return html! {
                <div class="panel panel-div">
                    <h3 class="panel-title">
                        { "☀️ Weather ☀️" }
                        <button class="link-button" onclick={ctx.link().callback(|_| Msg::LoadWeatherData)}>
                            { "🔁" }
                        </button>
                    </h3>
                    <p style="color: red">{{ error }}</p>
                </div>
            }
        } else if self.loading {
            return html! {
                <div class="panel panel-div">
                    <h3 class="panel-title">
                        { "☀️ Weather ☀️" }
                        <button class="link-button" onclick={ctx.link().callback(|_| Msg::LoadWeatherData)}>
                            { "🔁" }
                        </button>
                    </h3>
                    <div v-if="loading" class="ring">
                        <div class="ball-holder">
                            <div class="ball"></div>
                        </div>
                    </div>
                </div>
            }
        }

        if let Some(weather) = &self.weather {
            let temperature = format!("{:.0}°C", weather.main.temp);
            let feel = format!("{:.0}°C", weather.main.feels_like);
            let min = format!("⬇️ {:.0}°C", weather.main.temp_min);
            let max = format!("⬆️ {:.0}°C", weather.main.temp_max);
    
            let weather_icon = format!("/img/owm/icons/{}@2x.png", weather.weather[0].icon);
            let weather_description = &weather.weather[0].description;

            let sun_time = {
                let sunrise = {
                    let timestamp = weather.sys.sunrise * 1000;
                    let sunrise = DateTime::from_timestamp(timestamp, 0);
                    if let Some(time) = sunrise {
                        format!("{:02}:{:02}", time.hour(), time.minute())
                    } else {
                        "N/A".to_string()
                    }
                };
                let sunset = {
                    let timestamp = weather.sys.sunset * 1000;
                    let sunset = DateTime::from_timestamp(timestamp, 0);
                    if let Some(time) = sunset {
                        format!("{:02}:{:02}", time.hour(), time.minute())
                    } else {
                        "N/A".to_string()
                    }
                };
                format!("🌅 {} 🌄 {}", sunrise, sunset)
            };

            let last_upd = {
                let plural = if self.time_since_last_update > 1 { "s" } else { "" };
                format!("{} minute{} ago", self.time_since_last_update, plural)
            };

            html! {
                <div class="panel panel-div">
                    <h3 class="panel-title">
                        { "☀️ Weather ☀️" }
                        <button class="link-button" onclick={ctx.link().callback(|_| Msg::LoadWeatherData)}>
                            { "🔁" }
                        </button>
                    </h3>
    
                    <h3 class="section-separator-title">{ "🌡️ Temperature 🌡️" }</h3>
                    <div>
                        <p class="central-content">{ temperature }</p>
                        <div class="small-grid">
                            <p class="small-grid-elem">{ feel }</p>
                            <p class="small-grid-elem center">{ min }</p>
                            <p class="small-grid-elem">{ max }</p>
                        </div>
                    </div>
                    <h3 class="section-separator-title">{ "☀️ Weather ☀️" }</h3>
                    <div style="text-align: center;width: 100%;">
                        <img
                            class="central-content"
                            style="max-height: 64px;"
                            src={ weather_icon }
                            alt="weather icon"
                        />
                        <p>{ weather_description }</p>
                    </div>
                    <h3 class="section-separator-title">{ "🌕 Day time ☀️" }</h3>
                    <div style="text-align: center;width: 100%;">
                        <p>{ sun_time }</p>
                    </div>
                    <small style="font-size: 0.7em;">
                        { last_upd }
                    </small>
                </div>
            }
        } else {
            html! {
                <div class="panel panel-div">
                    <h3 class="panel-title">
                        { "☀️ Weather ☀️" }
                        <button class="link-button" onclick={ctx.link().callback(|_| Msg::LoadWeatherData)}>
                            { "🔁" }
                        </button>
                    </h3>
                    <p>{ "No weather data available" }</p>
                </div>
            }
        }
        
    }
}