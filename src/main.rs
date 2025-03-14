mod app;

fn main() {
    // The actual initialization is now handled in lib.rs
    // This is just an entry point for non-WASM builds
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(app::app);
}
