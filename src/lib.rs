#[cfg(feature = "python-extension")]
use pyo3::prelude::*;

use wasm_bindgen::prelude::*;

// JavaScript functions
#[wasm_bindgen]
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

#[wasm_bindgen]
pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

#[wasm_bindgen]
pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

#[wasm_bindgen]
pub fn divide(a: f64, b: f64) -> Result<f64, JsValue> {
    if b == 0.0 {
        Err(JsValue::from_str("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

#[cfg(feature = "python-extension")]
#[pyfunction]
fn py_add(a: f64, b: f64) -> f64 {
    a + b
}

#[cfg(feature = "python-extension")]
#[pyfunction]
fn py_subtract(a: f64, b: f64) -> f64 {
    a - b
}

#[cfg(feature = "python-extension")]
#[pyfunction]
fn py_multiply(a: f64, b: f64) -> f64 {
    a * b
}

#[cfg(feature = "python-extension")]
#[pyfunction]
fn py_divide(a: f64, b: f64) -> PyResult<f64> {
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
fn calculator(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_add, m)?)?;
    m.add_function(wrap_pyfunction!(py_subtract, m)?)?;
    m.add_function(wrap_pyfunction!(py_multiply, m)?)?;
    m.add_function(wrap_pyfunction!(py_divide, m)?)?;
    Ok(())
}
