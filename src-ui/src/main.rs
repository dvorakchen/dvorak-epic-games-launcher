mod app;
mod components;
mod server;
mod storages;
mod utils;

use app::App;
use leptos::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! { <App/> }
    });
}
