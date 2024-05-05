use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = getSites, catch)]
    pub async fn get_sites() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = getDepartures, catch)]
    pub async fn get_departures(site: String) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = getCurrency, catch)]
    pub async fn get_currency() -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_name = getWeather, catch)]
    pub async fn get_weather() -> Result<JsValue, JsValue>;
}
