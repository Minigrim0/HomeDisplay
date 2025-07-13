use chrono::prelude::{DateTime, Local};
use futures::StreamExt;
use yew::{html, Component, Context, Html, Properties};

use super::services::{refresh_currency, start_currency_job, stream_time};
use homedisplay::models::currency::Conversion;

pub struct CurrencyComponent {
    currency: Option<Conversion>,
    loading: bool,
    error: Option<String>,
    current_date: DateTime<Local>,
}

pub enum Msg {
    ClockUpdate(DateTime<Local>),
    LoadCurrencyData,
    CurrencyDataReceived(Result<Conversion, String>),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub must_refresh: bool,
}

impl Component for CurrencyComponent {
    type Message = Msg;
    type Properties = Props;

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
        if ctx.props().must_refresh {
            ctx.link().send_message(Msg::LoadCurrencyData);
        }

        match msg {
            Msg::ClockUpdate(time) => {
                self.current_date = time;
                true
            }
            Msg::LoadCurrencyData => {
                refresh_currency(ctx.link().callback(Msg::CurrencyDataReceived));
                self.loading = true;
                self.error = None;
                self.currency = None;
                true
            }
            Msg::CurrencyDataReceived(result) => {
                match result {
                    Ok(value) => {
                        self.error = None;
                        self.currency = Some(value);
                    }
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
        let current_day: String = self.current_date.format("%A").to_string();
        let current_date: String = self.current_date.format("%d/%m/%Y").to_string();
        let current_time: String = self.current_date.format("%H:%M").to_string();

        let title_node = html! {
            <div style="width: 100%;text-align: center;">
            <p class="time-text">{ current_time }</p>
                <p class="date-text">{ current_day }</p>
                <p class="date-text">{ current_date }</p>
            </div>
        };

        if let Some(error) = &self.error {
            html! {
                <div class="panel">
                   { title_node }
                    <div class="panel-div">
                        <div>
                            <p style="color: red">{{ error }}</p>
                        </div>
                    </div>
                </div>
            }
        } else if self.loading {
            html! {
                <div class="panel">
                    { title_node }
                    <div class="panel-div">
                        <div class="ring">
                            <div class="ball-holder">
                                <div class="ball"></div>
                            </div>
                        </div>
                    </div>
                </div>
            }
        } else if let Some(conversion) = self.currency.as_ref() {
            let from_currency = format!(
                "{:.02} {}",
                conversion.from_currency_amount, conversion.from_currency
            );
            let to_currency = format!(
                "{:.02} {}",
                conversion.to_currency_amount, conversion.to_currency
            );

            let refresh_date = {
                let date_fetched = DateTime::from_timestamp(conversion.timestamp, 0)
                    .unwrap()
                    .with_timezone(&Local);
                let date = format!("{}", date_fetched.format("%d/%m/%Y"));
                let time = format!("{}", date_fetched.format("%H:%M"));
                format!("last update {date} {time}")
            };

            html! {
                <div class="panel">
                    { title_node }
                    <div class="panel-div">
                        <div>
                            <div>
                                <p class="currency-text">
                                    { from_currency }
                                </p>
                                <p class="currency-text">
                                    { to_currency }
                                </p>
                            </div>
                        </div>
                    </div>
                    <small class="refresh-text">
                        { refresh_date }
                    </small>
                </div>
            }
        } else {
            html! {
                <div class="panel">
                    { title_node }
                    <div class="panel-div">
                        <p>{ "No currency data available" }</p>
                    </div>
                </div>
            }
        }
    }
}
