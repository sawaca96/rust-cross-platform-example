#[cfg(all(target_arch = "wasm32", not(feature = "python-extension")))]
use wasm_bindgen::prelude::*;

#[cfg(feature = "python-extension")]
use pyo3::prelude::*;

// JavaScript functions
#[cfg(all(target_arch = "wasm32", not(feature = "python-extension")))]
#[wasm_bindgen]
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

#[cfg(all(target_arch = "wasm32", not(feature = "python-extension")))]
#[wasm_bindgen]
pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

#[cfg(all(target_arch = "wasm32", not(feature = "python-extension")))]
#[wasm_bindgen]
pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

#[cfg(all(target_arch = "wasm32", not(feature = "python-extension")))]
#[wasm_bindgen]
pub fn divide(a: f64, b: f64) -> Result<f64, JsValue> {
    if (b == 0.0) {
        Err(JsValue::from_str("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

// Python functions
#[cfg(feature = "python-extension")]
#[pyfunction]
fn add(a: f64, b: f64) -> f64 {
    a + b
}

#[cfg(feature = "python-extension")]
#[pyfunction]
fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

#[cfg(feature = "python-extension")]
#[pyfunction]
fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

#[cfg(feature = "python-extension")]
#[pyfunction]
fn divide(a: f64, b: f64) -> PyResult<f64> {
    if b == 0.0 {
        Err(pyo3::exceptions::PyValueError::new_err(
            "Cannot divide by zero",
        ))
    } else {
        Ok(a / b)
    }
}

#[cfg(feature = "python-extension")]
#[pymodule]
fn calculator(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(subtract, m)?)?;
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_function(wrap_pyfunction!(divide, m)?)?;
    Ok(())
}
