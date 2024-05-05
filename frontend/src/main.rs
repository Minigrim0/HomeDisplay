use yew::prelude::*;

mod glue;
mod weather;

use weather::weather_component::WeatherComponent;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    // Render the WeatherComponent component
    html! {
        <div>
            <h1>{ "Home Display" }</h1>
            <WeatherComponent />
        </div>
    }
}
