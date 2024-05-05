use chrono::{format, prelude::{DateTime, Local, Timelike}};
use futures::StreamExt;
use yew::{html, Html, Component, Context};

use super::services::{start_currency_job, stream_time, refresh_currency};
use common::models::currency::Conversion;


pub struct CurrencyComponent {
    currency: Option<Conversion>,
    loading: bool,
    error: Option<String>,
    current_date: DateTime<Local>,
}

pub enum Msg {
    ClockUpdate(DateTime<Local>),
    LoadCurrencyData,
    CurrencyDataReceived(Result<Conversion, String>)
}


impl Component for CurrencyComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let currency_ready_cb = ctx.link().callback(Msg::CurrencyDataReceived);
        start_currency_job(currency_ready_cb);

        let time_stream = stream_time();
        ctx.link().send_stream(time_stream.map(Msg::ClockUpdate));

        Self {
            currency: None,
            loading: true,
            error: None,
            current_date: Local::now(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ClockUpdate(time) => {
                self.current_date = time; 
                true
            },
            Msg::LoadCurrencyData => {
                refresh_currency(ctx.link().callback(Msg::CurrencyDataReceived));
                self.loading = true;
                self.error = None;
                self.currency = None;
                true
            },
            Msg::CurrencyDataReceived(result) => {
                match result {
                    Ok(value) => {
                        self.error = None;
                        self.currency = Some(value);
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
        let current_date = self.current_date.format("%d/%m/%Y").to_string();
        let current_time = self.current_date.format("%H:%M:%S").to_string();

        let title_node = html! {
            <div style="width: 100%;text-align: center;">
                <h1>{ "Home Display" }</h1>
                <p class="bot-text">{ current_date }</p>
                <p class="top-text">{ current_time }</p>
            </div>
        };

        let panel_title = html! {
            <h3 class="panel-title">
                { "💲 Currency 💲" }
                <button class="link-button" onclick={ctx.link().callback(|_| Msg::LoadCurrencyData)}>{ "🔁" }</button>
            </h3>
        };

        if let Some(error) = &self.error {
            html! {
                <div class="panel">
                   { title_node } 
                    <div class="panel-div">
                        { panel_title }
                        <div>
                            <p style="color: red">{{ error }}</p>
                        </div>
                    </div>
                </div>
            }
        }
        else if self.loading {
            html! {
                <div class="panel">
                    { title_node }
                    <div class="panel-div">
                        { panel_title }
                        <div class="ring">
                            <div class="ball-holder">
                                <div class="ball"></div>
                            </div>
                        </div>
                    </div>
                </div>
            }
        } else if let Some(conversion) = self.currency.as_ref() {
            let from_currency = format!("{:.02} {}", conversion.from_currency_amount, conversion.from_currency);
            let to_currency = format!("{:.02} {}", conversion.to_currency_amount, conversion.to_currency);
            
            let refresh_date = {
                let date_fetched = DateTime::from_timestamp(conversion.timestamp * 1000, 0).unwrap();
                let date = format!("{}", date_fetched.format("%d/%m/%Y"));
                let time = format!("{}", date_fetched.format("%H:%M:%S"));
                format!("last update {date} {time}");
            };

            html! {
                <div class="panel">
                    { title_node }
                    <div class="panel-div">
                        { panel_title }
                        <div> 
                            <div>
                                <p style="text-align: center;">
                                    <span style="border: 2px solid whitesmoke;padding: 0.2em;border-radius: 5px;">
                                        { from_currency }
                                    </span>
                                    { "=" }
                                    <span style="border: 2px solid whitesmoke;padding: 0.2em;border-radius: 5px;">
                                        { to_currency }
                                    </span>
                                </p>
                                <small style="font-size: 0.7em;">
                                    { refresh_date }
                                </small>
                            </div>
                        </div>
                    </div>
                </div>
            }
        }
        else {
            html! {
                <div class="panel">
                    { title_node }
                    <div class="panel-div">
                        { panel_title }
                        <p>{ "No currency data available" }</p>
                    </div>
                </div>
            }
        }


    }
}