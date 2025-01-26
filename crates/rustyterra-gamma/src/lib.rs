mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

async fn run_async() -> Result<(), JsValue> {
  console_log!("Hello from Rust WASM!");
  Ok(())
}

#[wasm_bindgen(start)]
fn run() {
  spawn_local(async {
    run_async().await.unwrap_throw();
  });
}
