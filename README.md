# pulldown-cmark-wasm

A simple WebAssembly wrapper for the excellent [pulldown-cmark](https://crates.io/crates/pulldown-cmark) parser for the standardized version of Markdown, CommonMark.

When packaged with [wasm-pack](https://rustwasm.github.io/wasm-pack/), it creates a JS module exporting a single function, `format`. Pass in a string containing Markdown formatted text, get out CommonMark-compliantly generated html—that simple.
