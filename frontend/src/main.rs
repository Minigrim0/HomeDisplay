use yew::prelude::*;

mod glue;
mod weather;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <h2 class={"heading"}>{"Home Display"}</h2>
        </div>
    }
}
