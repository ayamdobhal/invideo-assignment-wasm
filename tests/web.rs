//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen_test::*;

use invideo_assignment_wasm::evaluate_expression;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn addition() {
    let expr = "34 + 35".to_string();
    assert_eq!(evaluate_expression(&expr), "69");
}

#[wasm_bindgen_test]
fn subtraction() {
    let expr = "121 - 20".to_string();
    assert_eq!(evaluate_expression(&expr), "101");
}

#[wasm_bindgen_test]
fn multiplication() {
    let expr = "35 * 2".to_string();
    assert_eq!(evaluate_expression(&expr), "70");
}

#[wasm_bindgen_test]
fn division() {
    let expr = "15 / 3".to_string();
    assert_eq!(evaluate_expression(&expr), "5");
}

#[wasm_bindgen_test]
fn complex_expr() {
    let expr = "2 * (3 + 60 / (5 + 9))".to_string();
    assert_eq!(evaluate_expression(&expr), "14.571428571428571");
}

#[wasm_bindgen_test]
fn empty_expression() {
    let expr = "".to_string();
    assert_eq!(evaluate_expression(&expr), "Invalid Expression");
}

#[wasm_bindgen_test]
fn invalid_expression() {
    let expr = "lol".to_string();
    assert_eq!(evaluate_expression(&expr), "Invalid Expression");
}
