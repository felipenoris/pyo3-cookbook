use pyo3::prelude::*;
use numpy::ndarray::{ArrayD, ArrayViewD};
use numpy::{IntoPyArray, PyArrayDyn, PyReadonlyArrayDyn};

use arrow::record_batch::RecordBatch;
use pyo3_arrow::PyRecordBatch;

use std::sync::Arc;

use arrow::array::{Array, ArrayRef, Float64Array, Float64Builder};
use arrow::datatypes::{DataType, Field, Schema};

mod plano_contas;

#[pyfunction]
fn process_arrow_batch(batch: PyRecordBatch) -> PyResult<()> {

    let batch: RecordBatch = batch.into();

    println!("Linhas: {}", batch.num_rows());

    Ok(())
}

#[pyfunction]
fn process_arrow_batch_mult2(batch: PyRecordBatch, col_name: &str) -> PyResult<PyRecordBatch> {

    let batch: RecordBatch = batch.into();

    // Obtém a coluna passada por parâmetro
    let valores = batch
        .column_by_name(col_name)
        .expect("Não encontrou coluna col_name")
        .as_any()
        .downcast_ref::<Float64Array>()
        .expect("não conseguiu realizar downcast para Float64");

    // Constrói uma nova coluna
    let mut builder = Float64Builder::with_capacity(valores.len());

    for i in 0..valores.len() {
        if valores.is_null(i) {
            builder.append_null();
        } else {
            builder.append_value(valores.value(i) * 2.0);
        }
    }

    let valor_dobrado: ArrayRef = Arc::new(builder.finish());

    // Novo schema = schema antigo + nova coluna
    let mut fields = batch.schema().fields().to_vec();

    fields.push(Arc::new(Field::new(
        "valor_dobrado",
        DataType::Float64,
        true,
    )));

    let schema = Arc::new(Schema::new(fields));

    // Colunas antigas
    let mut columns = batch.columns().to_vec();

    // Acrescenta a nova
    columns.push(valor_dobrado);

    // Cria novo RecordBatch
    let novo_batch =
        RecordBatch::try_new(schema, columns)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;

    // Converte Arrow -> PyArrow
    Ok(novo_batch.into())
}

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
    m.add_function(wrap_pyfunction!(process_arrow_batch, m)?)?;
    m.add_function(wrap_pyfunction!(process_arrow_batch_mult2, m)?)?;
    m.add_class::<plano_contas::Conta>()?;
    m.add_class::<plano_contas::Veiculo>()?;
    m.add_class::<plano_contas::RelacionamentoContaHierarquia>()?;
    m.add_class::<plano_contas::HierarquiaContas>()?;
    m.add_class::<plano_contas::Lancamento>()?;
    Ok(())
}
