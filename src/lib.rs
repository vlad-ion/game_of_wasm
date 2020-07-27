mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
	//let greeting = format!("Hello, {} to game of wasm 💖!", name);
    
	let greeting1 = String::from("Hello, ");
	let greeting2 = greeting1 + name;
	let greeting = greeting2 + " to game of wasm 💖!";
	
	alert(&greeting);
}

