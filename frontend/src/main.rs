use yew::prelude::*;

mod glue;
mod weather;
mod currency;

use weather::weather_component::WeatherComponent;
use currency::currency_component::CurrencyComponent;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    // Render the WeatherComponent component
    html! {
        <div class="container">
            <WeatherComponent />
            <CurrencyComponent />
        </div>
    }
}
