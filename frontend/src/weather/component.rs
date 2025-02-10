use yew::{html, Component, Context, Html, Properties};
use chrono::prelude::{Local, Timelike};
use futures::StreamExt;

use common::models::weather::WeatherInfo;

use super::services::{start_weather_job, refresh_weather, stream_time};

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

#[derive(Properties, PartialEq)]
pub struct Props {
    pub must_refresh: bool,
}

impl Component for WeatherComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::LoadWeatherData);

        let weather_ready_cb = ctx.link().callback(Msg::WeatherDataReceived);
        start_weather_job(weather_ready_cb);

        let time_stream = stream_time();
        ctx.link().send_stream(time_stream.map(|_| Msg::ClockUpdate));

        Self {
            weather: None,
            loading: false,
            error: None,
            last_update: Local::now().timestamp(),
            time_since_last_update: 0
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        if ctx.props().must_refresh {
            ctx.link().send_message(Msg::LoadWeatherData);
        }

        match msg {
            Msg::ClockUpdate => {
                let now = Local::now().timestamp();
                self.time_since_last_update = now - self.last_update;
                true
            }
            Msg::LoadWeatherData => {
                refresh_weather(ctx.link().callback(Msg::WeatherDataReceived));
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

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if let Some(error) = &self.error {
            html! {
                <div class="panel panel-div">
                    <p style="color: red">{{ error }}</p>
                </div>
            }
        } else if self.loading {
            html! {
                <div class="panel panel-div">
                    <div v-if="loading" class="ring">
                        <div class="ball-holder">
                            <div class="ball"></div>
                        </div>
                    </div>
                </div>
            }
        } else if let Some(weather) = &self.weather {
            let mut errors = vec![];

            let temperature = format!("{:.0}°C", weather.current.temperature_2m);
            let feel = format!("Feel {:.0}°C", weather.current.apparent_temperature);
            let min = format!("⬇️ {:.0}°C", weather.daily.temperature_2m_min.first().unwrap_or(&-1000.0));
            let max = format!("⬆️ {:.0}°C", weather.daily.temperature_2m_max.first().unwrap_or(&1000.0));

            let (icon_code, weather_description) = weather.daily.get_weather_info()
                .map_err(|e| errors.push(format!("Unable to get weather info: {}", e.to_string())))
                .unwrap_or(("01d".to_string(), "error".to_string()));

            let weather_icon = format!("/static/owm/icons/{}@2x.png", icon_code);

            let (sunrise, sunset, daytime) = match weather.daily.get_sun_info() {
                Ok((sr, ss, dt)) => (sr, ss, dt),
                Err(e) => {
                    errors.push(format!("Unable to get sun data: {}", e.to_string()));
                    (Local::now().fixed_offset(), Local::now().fixed_offset(), 0.0)
                }
            };

            let sun_time = {
                let daytime = {
                    let daytime = daytime as i32;
                    let hours = daytime / 3600;
                    let minutes = (daytime - hours * 3600) / 60;
                    let seconds = daytime % 60;
                    format!("{:02}h {:02}m {:02}s", hours, minutes, seconds)
                };
                let sunrise = {
                    format!("{:02}:{:02}", sunrise.hour(), sunrise.minute())
                };
                let sunset = {
                    format!("{:02}:{:02}", sunset.hour(), sunset.minute())
                };
                format!("🌅 {} 🌄 {} ({})", sunrise, sunset, daytime)
            };

            let forecast = match weather.daily.get_forecast() {
                Ok(f) => f,
                Err(e) => {
                    errors.push(format!("Unable to get forecast: {}", e.to_string()));
                    vec![]
                }
            };

            let last_upd = {
                let plural = if (self.time_since_last_update / 60) > 1 { "s" } else { "" };
                format!("{} minute{} ago", (self.time_since_last_update / 60) as i32, plural)
            };

            html! {
                <div class="panel panel-div">
                    <div>
                        <p class="central-content">{ temperature }</p>
                        <div class="small-grid">
                            <p class="small-grid-elem">{ feel }</p>
                            <p class="small-grid-elem center">{ min }</p>
                            <p class="small-grid-elem">{ max }</p>
                        </div>
                    </div>
                    <h3 class="section-separator-title"></h3>
                    <div style="text-align: center;width: 100%;">
                        <img
                            class="central-content"
                            style="max-height: 64px;"
                            src={ weather_icon }
                            alt="weather icon"
                        />
                        <p>{ weather_description }</p>
                    </div>
                    <h3 class="section-separator-title">{ "Forecast" }</h3>
                    <div style="max-height: 25vh;overflow-y: scroll">
                        <table>
                            <tr>
                                <th></th>
                                <th></th>
                                <th>{"min"}</th>
                                <th>{"max"}</th>
                                <th>{"uv"}</th>
                                <th>{"feel min"}</th>
                                <th>{"feel max"}</th>
                            </tr>
                            { forecast.iter().map(|f| html! {
                                <tr>
                                    <td><p>{ f.time.format("%a %d").to_string() }</p></td>
                                    <td><img src={ format!("/static/owm/icons/{:02}@2x.png", f.weather_code) } alt="weather icon" style="max-height: 30px;"/></td>
                                    <td><p>{ format!("{:.0}°C", f.temperature_2m_min) }</p></td>
                                    <td><p>{ format!("{:.0}°C", f.temperature_2m_max) }</p></td>
                                    <td><p>{ format!("{:.0}", f.uv_index_max) }</p></td>
                                    <td><p>{ format!("{:.0}°C", f.apparent_temperature_min) }</p></td>
                                    <td><p>{ format!("{:.0}°C", f.apparent_temperature_max) }</p></td>
                                </tr>
                            }).collect::<Html>() }
                        </table>
                    </div>
                    <h3 class="section-separator-title">{ "🌕 Day time ☀️" }</h3>
                    <div style="text-align: center;width: 100%;">
                        <p>{ sun_time }</p>
                    </div>
                    <small class="error-list">
                        { errors.iter().map(|e| html! { <p>{"Error: "}{ e }</p> }).collect::<Html>() }
                    </small>
                    <small class="refresh-text">
                        { last_upd }
                    </small>
                </div>
            }
        } else {
            html! {
                <div class="panel panel-div">
                    <p>{ "No weather data available" }</p>
                </div>
            }
        }
    }
}
