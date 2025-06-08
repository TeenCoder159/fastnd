/// Things that don't really fit anywhere else
pub mod utils;

use pyo3::prelude::*;
#[pymodule]
fn fastnd(m: &Bound<'_, PyModule>) -> PyResult<()> {
    utils::bindings::add_bindings(m)?;
    Ok(())
}
