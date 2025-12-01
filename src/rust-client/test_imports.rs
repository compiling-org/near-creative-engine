#[cfg(test)]
mod tests {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsValue;
    use js_sys::{Date, JSON, Array};
    use web_sys;

    #[test]
    fn test_imports() {
        let _time = Date::now();
        println!("Imports work correctly");
    }
}