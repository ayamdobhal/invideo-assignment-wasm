use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn evaluate_expression(expression: &str) -> String {
    match meval::eval_str(expression) {
        Ok(result) => result.to_string(),
        Err(_) => "Invalid Expression".to_string(),
    }
}
