use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use tea_yi::Hexagram;

/// Convert 6 bools (upper to lower) into hexagram info (kw index, name, unicode symbol).
#[pyfunction]
fn hexagram_info(bits: Vec<bool>) -> PyResult<(u8, String, String)> {
    if bits.len() != 6 {
        return Err(PyValueError::new_err("expected 6 booleans (upper to lower lines)"));
    }

    let hex = Hexagram::from_slice(&bits);
    Ok((hex.kw(), hex.name().to_string(), hex.unicode().to_string()))
}

/// Python module initializer.
#[pymodule]
fn py_yi(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hexagram_info, m)?)?;
    Ok(())
}
