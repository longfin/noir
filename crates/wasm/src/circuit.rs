use acvm::acir::circuit::Circuit;
use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen::prelude::*;

// Deserializes bytes into ACIR structure
#[deprecated(
    note = "we have moved away from this serialization strategy. Call `acir_read_bytes` instead"
)]
#[allow(deprecated)]
#[wasm_bindgen]
pub fn acir_from_bytes(bytes: Vec<u8>) -> JsValue {
    console_error_panic_hook::set_once();
    let circuit = Circuit::from_bytes(&bytes);
    <JsValue as JsValueSerdeExt>::from_serde(&circuit).unwrap()
}

#[deprecated(
    note = "we have moved away from this serialization strategy. Call `acir_write_bytes` instead"
)]
#[allow(deprecated)]
#[wasm_bindgen]
pub fn acir_to_bytes(acir: JsValue) -> Vec<u8> {
    console_error_panic_hook::set_once();
    let circuit: Circuit = JsValueSerdeExt::into_serde(&acir).unwrap();
    circuit.to_bytes()
}

// Deserializes bytes into ACIR structure
#[wasm_bindgen]
pub fn acir_read_bytes(bytes: Vec<u8>) -> JsValue {
    console_error_panic_hook::set_once();
    let circuit = Circuit::read(&*bytes).unwrap();
    <JsValue as JsValueSerdeExt>::from_serde(&circuit).unwrap()
}

#[wasm_bindgen]
pub fn acir_write_bytes(acir: JsValue) -> Vec<u8> {
    console_error_panic_hook::set_once();
    let circuit: Circuit = JsValueSerdeExt::into_serde(&acir).unwrap();
    let mut bytes = Vec::new();
    circuit.write(&mut bytes).unwrap();
    bytes
}
