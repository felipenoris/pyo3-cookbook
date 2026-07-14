use chrono::NaiveDate;
use rust_decimal::Decimal;
use pyo3::prelude::*;
//use std::collections::HashMap;
//use serde_json::Value;

//type JsonMap = HashMap<String, Value>;

#[pyclass]
pub struct Conta {
    #[pyo3(get, set)]
    id_conta: i64,
    #[pyo3(get, set)]
    nome: String,
    #[pyo3(get, set)]
    codigo: String,
    #[pyo3(get, set)]
    permite_lancamentos: bool,
}

#[pyclass]
pub struct Veiculo {
    #[pyo3(get, set)]
    id_veiculo: i64,
    #[pyo3(get, set)]
    nome: String,
}

#[pymethods]
impl Veiculo {

    #[new]
    pub fn new(id_veiculo: i64, nome: String) -> Self {
        Self {
            id_veiculo,
            nome,
        }
    }
}

#[pyclass]
pub struct RelacionamentoContaHierarquia {
    #[pyo3(get, set)]
    id_rel_conta_hierarquia: i64,
    #[pyo3(get, set)]
    id_hierarquia: i64,
    #[pyo3(get, set)]
    id_parent: i64,
    #[pyo3(get, set)]
    id_child: i64,
}

#[pyclass]
pub struct HierarquiaContas {
    #[pyo3(get, set)]
    id_hierarquia: i64,
    #[pyo3(get, set)]
    nome: String,
    #[pyo3(get, set)]
    descricao: Option<String>,
}

#[pyclass]
pub struct Lancamento {
    #[pyo3(get, set)]
    id_lancamento: i64,
    #[pyo3(get, set)]
    id_veiculo: i64,
    #[pyo3(get, set)]
    id_conta: i64,
    #[pyo3(get, set)]
    data: NaiveDate,
    #[pyo3(get, set)]
    valor: Decimal,
    //meta: Option<Value>,
    #[pyo3(get, set)]
    meta: String,
}
