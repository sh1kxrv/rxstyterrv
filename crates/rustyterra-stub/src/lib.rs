mod utils;

use js_sys::{Function, Object, Reflect, WebAssembly};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{spawn_local, JsFuture};

const WASM: &[u8] = include_bytes!("rustyterra_gamma_bg.wasm");

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

async fn run_async() -> Result<(), JsValue> {
  console_log!("instantiating a new Wasm module directly");

  let a = JsFuture::from(WebAssembly::instantiate_buffer(WASM, &Object::new())).await?;
  let b: WebAssembly::Instance = Reflect::get(&a, &"instance".into())?.dyn_into()?;
  let c = b.exports();

  let run = Reflect::get(c.as_ref(), &"run".into())?
    .dyn_into::<Function>()
    .expect("run export wasn't a function");

  run.call0(&JsValue::undefined())?;

  // let three = add.call2(&JsValue::undefined(), &1.into(), &2.into())?;
  // console_log!("1 + 2 = {:?}", three);
  // let mem = Reflect::get(c.as_ref(), &"memory".into())?
  //   .dyn_into::<WebAssembly::Memory>()
  //   .expect("memory export wasn't a `WebAssembly.Memory`");
  // console_log!("created module has {} pages of memory", mem.grow(0));
  // console_log!("giving the module 4 more pages of memory");
  // mem.grow(4);
  // console_log!("now the module has {} pages of memory", mem.grow(0));

  Ok(())
}

#[wasm_bindgen(start)]
fn run() {
  spawn_local(async {
    run_async().await.unwrap_throw();
  });
}
