use crate::utils::utils::zeroes;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction(name = "zeros")]
fn zeros_binding(n: usize) -> Vec<isize> {
    zeroes(n)
}

/// Add bindings. Internal.
pub(crate) fn add_bindings(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(zeros_binding, m)?)
}
