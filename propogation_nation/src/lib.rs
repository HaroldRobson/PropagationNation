pub mod app;
pub mod home;
pub mod inputpages;
pub mod prelude;
pub mod types;
pub mod utils;
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
