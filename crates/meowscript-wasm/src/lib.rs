//! Browser entry point, built with `wasm-pack build --target web`. The playground calls `run`
//! from a Web Worker so an endless loop can be stopped by terminating the worker.

use meowscript::{Host, Interpreter};
use wasm_bindgen::prelude::*;

struct JsHost {
    on_output: js_sys::Function,
}

impl Host for JsHost {
    fn print(&mut self, line: &str) {
        let _ = self
            .on_output
            .call1(&JsValue::NULL, &JsValue::from_str(line));
    }

    // Workers have no blocking sleep. Spinning is fine off the main thread, and output already
    // posted keeps flowing.
    fn sleep(&mut self, millis: f64) {
        let end = js_sys::Date::now() + millis;
        while js_sys::Date::now() < end {
            std::hint::spin_loop();
        }
    }

    fn now_millis(&mut self) -> f64 {
        js_sys::Date::now()
    }

    fn read_line(&mut self, _prompt: &str) -> meowscript::Result<String> {
        Err(meowscript::Error::io(
            "`kibble` can't read input in the browser playground yet",
        ))
    }

    fn random_seed(&mut self) -> u64 {
        let time = js_sys::Date::now() as u64;
        let noise = (js_sys::Math::random() * (1u64 << 53) as f64) as u64;
        time.rotate_left(17) ^ noise
    }
}

/// Calls `on_output` once per printed line. Rejects with the rendered error text on failure.
#[wasm_bindgen]
pub fn run(source: &str, on_output: &js_sys::Function) -> Result<(), JsValue> {
    let host = JsHost {
        on_output: on_output.clone(),
    };
    let mut interp = Interpreter::new(host);
    interp
        .run(source)
        .map(|_| ())
        .map_err(|e| JsValue::from_str(&e.render(source, None)))
}

#[wasm_bindgen]
pub fn version() -> String {
    meowscript::VERSION.to_string()
}

#[wasm_bindgen]
pub fn keywords() -> Vec<String> {
    meowscript::token::TokenKind::KEYWORDS
        .iter()
        .map(|k| k.to_string())
        .collect()
}
