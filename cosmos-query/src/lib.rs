use wasm_bindgen::prelude::*;

#[wasm_bindgen]
unsafe extern {
    pub unsafe fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    unsafe {    
    alert(&format!("hello, {}!", name));
    }
}
