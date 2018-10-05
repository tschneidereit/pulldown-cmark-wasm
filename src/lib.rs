extern crate cfg_if;
extern crate pulldown_cmark;
extern crate wasm_bindgen;

mod utils;

use pulldown_cmark::{html, Parser};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn format(markdown: &str) -> String {
    let mut html_buf = String::new();
    let parser = Parser::new(markdown);
    html::push_html(&mut html_buf, parser);
    html_buf
}
