use pyo3::prelude::*;
use pyo3_polars::derive::polars_expr;
use pyo3_polars::export::polars_core::prelude::*;
use pyo3_polars::export::polars_error::PolarsResult;
use pyo3_polars::types::*;
use serde::Deserialize;
use tea_yi::Hexagram;

macro_rules! auto_cast {
    // for one expression
    ($arm: ident ($se: expr)) => {
        if let DataType::$arm = $se.dtype() {
            $se
        } else {
            &$se.cast(&DataType::$arm)?
        }
    };
    // for multiple expressions
    ($arm: ident ($($se: expr),*)) => {
        ($(
            if let DataType::$arm = $se.dtype() {
                $se
            } else {
                &$se.cast(&DataType::$arm)?
            }
        ),*)
    };
}

#[derive(Deserialize)]
pub struct YiParams {
    rev: bool,
}

#[polars_expr(output_type=String)]
fn evaluators_future_dirty_price(inputs: &[Series], kwargs: YiParams) -> PolarsResult<Series> {
    let series = auto_cast!(Boolean(&inputs[0]));
    series.bool()?;
    todo!();
}

/// Python module initializer.
#[pymodule]
fn py_yi(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(hexagram_info, m)?)?;
    Ok(())
}
