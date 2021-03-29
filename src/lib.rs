use wasm_bindgen::prelude::*;

#[cfg(feature="wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("{}", name));
}

#[wasm_bindgen]
pub fn variable()-> usize{
    let space = "     ";
    let space: usize = space.len();
    return space;
}

#[wasm_bindgen]
pub fn ieee() -> char{
    let x: char = 'c';
    return x;
}

#[wasm_bindgen]
pub fn tuple() -> i32 {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    return x;
}

#[wasm_bindgen]
pub fn arr() -> i32{
    let a = [1, 2, 3, 4, 5];
    return a[0];
}