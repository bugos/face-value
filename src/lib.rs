mod app;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    // Launch the app
    dioxus_web::launch(app::app);
}
