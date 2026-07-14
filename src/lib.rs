use pyo3::prelude::*;
use numpy::ndarray::{ArrayD, ArrayViewD, ArrayViewMutD};
use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArrayDyn, PyArrayMethods};

mod plano_contas;

#[pyfunction]
fn triple(x: usize) -> usize {
    3 * x
}

// example using immutable borrows producing a new array
fn axpy(a: f64, x: ArrayViewD<'_, f64>, y: ArrayViewD<'_, f64>) -> ArrayD<f64> {
    a * &x + &y
}

// wrapper of `axpy`
#[pyfunction(name = "axpy")]
fn axpy_py<'py>(
    py: Python<'py>,
    a: f64,
    x: PyReadonlyArrayDyn<'py, f64>,
    y: PyReadonlyArrayDyn<'py, f64>,
) -> Bound<'py, PyArrayDyn<f64>> {
    let x = x.as_array();
    let y = y.as_array();
    let z = axpy(a, x, y);
    z.into_pyarray(py)
}

#[pymodule]
fn _core(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(triple, m)?)?;
    m.add_function(wrap_pyfunction!(axpy_py, m)?)?;
    m.add_class::<plano_contas::Conta>()?;
    m.add_class::<plano_contas::Veiculo>()?;
    m.add_class::<plano_contas::RelacionamentoContaHierarquia>()?;
    m.add_class::<plano_contas::HierarquiaContas>()?;
    m.add_class::<plano_contas::Lancamento>()?;
    Ok(())
}
