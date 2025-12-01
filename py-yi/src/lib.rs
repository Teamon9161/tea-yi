use pyo3::prelude::*;
use pyo3_polars::derive::polars_expr;
use pyo3_polars::export::polars_core::prelude::*;
use pyo3_polars::export::polars_error::PolarsResult;
use serde::Deserialize;
use tea_yi::Hexagram;
use tevec::prelude::Vec1View;

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
fn into_hexagram(inputs: &[Series], kwargs: YiParams) -> PolarsResult<Series> {
    let series = auto_cast!(Boolean(&inputs[0]));
    let out: Vec<_> = series
        .bool()?
        .rolling_custom(
            6,
            |s| {
                let h = if !kwargs.rev {
                    Hexagram::from_iter(s.iter().map(|b| b.unwrap()))
                } else {
                    Hexagram::from_iter(s.iter().rev().map(|b| b.unwrap()))
                };
                format!("{}", h.name())
            },
            None,
        )
        .unwrap();
    Ok(StringChunked::from_iter_values("".into(), out.into_iter()).into_series())
}

/// Python module initializer.
#[pymodule]
fn py_yi(_py: Python<'_>, _m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
