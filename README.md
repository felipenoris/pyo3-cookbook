# pyo3-cookbook

# Data Pipeline Structure

```
source (database / excel file / parquet / redshift UNLOAD)
↓
batch of lines
↓
Python: DataFrame with pyarrow backend
↓
Arrow Table / Arrow RecordBatch
↓
Rust: pyo3-arrow + Arrow RecordBatch
↓
Rust: in-memory data structures with business logic
↓
Rust: returns Arrow RecordBatch to Python
↓
Python: writes parquet file from Arrow RecordBatch
↓
Redshift: COPY parquet table from S3 file
```

```python
pipeline.read_sql(sql)
    .rust(etl_fun_logic)
    .write_redshift("result.parquet")
```

```python
class SqlReader:

    def batches(self, sql):

        for df in pd.read_sql(
                sql,
                self.engine,
                chunksize=100_000,
                dtype_backend="pyarrow"):

            table = pa.Table.from_pandas(df)

            yield from table.to_batches()
```

```python
for batch in reader.batches(sql):
    rust.process(batch)
```

```rust
use arrow::record_batch::RecordBatch;
use pyo3::prelude::*;
use pyo3_arrow::PyRecordBatch;

#[pyfunction]
fn process(batch: PyRecordBatch) -> PyResult<()> {

    let batch: RecordBatch = batch.into();

    println!("Linhas: {}", batch.num_rows());

    Ok(())
}
```

```rust
use arrow::array::Float64Array;

let coluna = batch.column(3);

let valores = coluna
    .as_any()
    .downcast_ref::<Float64Array>()
    .unwrap();

for i in 0..valores.len() {

    let valor = valores.value(i);

    println!("{valor}");

}
```

Ou, de forma mais abstrata:

```rust
#[pyfunction]
fn process(batch: PyRecordBatch) -> PyResult<()> {

    let batch: RecordBatch = batch.into();

    LancamentoReader::new(batch)
        .process()?;

    Ok(())
}

pub struct LancamentoReader {

    batch: RecordBatch,

}

impl LancamentoReader {

    pub fn process(self) -> Result<()> {

        let lancamentos = self.read_lancamentos();

        // algoritmo financeiro

        Ok(())
    }

}

pub struct LancamentoBatch<'a> {

    datas: &'a Date32Array,

    contas: &'a StringArray,

    valores: &'a Float64Array,

}

let l = batch.get(i);
```

Ou, copiando valores:

```rust
struct Lancamento {

    data: i32,
    conta: String,
    valor: f64,

}

use arrow::array::{
    Date32Array,
    Float64Array,
    StringArray,
};

let datas = batch.column(0)
    .as_any()
    .downcast_ref::<Date32Array>()
    .unwrap();

let contas = batch.column(1)
    .as_any()
    .downcast_ref::<StringArray>()
    .unwrap();

let valores = batch.column(2)
    .as_any()
    .downcast_ref::<Float64Array>()
    .unwrap();

let mut lancamentos = Vec::with_capacity(batch.num_rows());

for i in 0..batch.num_rows() {

    lancamentos.push(Lancamento {

        data: datas.value(i),

        conta: contas.value(i).to_owned(),

        valor: valores.value(i),

    });

}
```

# master data vs objetos transacionais

Carregar em memória master data: contas, hierarquia de contas.

Utilizar batch para objetos transacionais, grande volume.

# References

- <https://pyo3.rs/v0.29.0/>

- <https://github.com/PyO3/rust-numpy>

- <https://github.com/pola-rs/polars>