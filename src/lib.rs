pub mod components;
pub mod reqs;

use components::main::Main;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub const HTML_HIDDEN: &str = "display:none;";
pub const HTML_VISIBLE: &str = "display:block;";

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Main>::new().mount_to_body();
}
