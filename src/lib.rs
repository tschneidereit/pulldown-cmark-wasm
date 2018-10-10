extern crate cfg_if;
extern crate pulldown_cmark;
extern crate wasm_bindgen;

mod utils;

use cfg_if::cfg_if;
use pulldown_cmark::{html, Parser};
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn format(markdown: &str) -> String {
    utils::set_panic_hook();
    let mut html_buf = String::new();
    let parser = Parser::new(markdown);
    html::push_html(&mut html_buf, parser);
    html_buf
}
