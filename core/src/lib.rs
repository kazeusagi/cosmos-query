use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// unsafe extern {
//     pub unsafe fn alert(s: &str);
//     pub unsafe fn print(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     unsafe {    
//     alert(&format!("hello, {}!", name));
//     }
// }

// #[wasm_bindgen]
// pub fn printwasm(name: &str) {
//     unsafe {    
//         print(name);
//     }
// }


#[wasm_bindgen]
pub fn add(num1: i32, num2:i32)-> i32 {
    return num1 + num2;
}
