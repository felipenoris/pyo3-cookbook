use pyo3::prelude::*;

#[pyfunction]
fn triple(x: usize) -> usize {
    3 * x
}

#[pymodule]
fn _core(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(triple, m)?)?;
    Ok(())
}
