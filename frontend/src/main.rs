use yew::prelude::*;

mod glue;
mod weather;
mod currency;
mod transports;

use weather::component::WeatherComponent;
use currency::component::CurrencyComponent;
use transports::components::transport::TransportsComponent;


fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    // Render the WeatherComponent component
    html! {
        <div class="container">
            <WeatherComponent must_refresh=false />
            <CurrencyComponent must_refresh=false />
            <TransportsComponent must_refresh=false />
        </div>
    }
}
