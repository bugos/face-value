mod app;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    // Initialize panic hook for better error messages
    console_error_panic_hook::set_once();

    // Initialize logging
    wasm_logger::init(wasm_logger::Config::default());

    // Launch the app
    dioxus_web::launch(app::app);
}
