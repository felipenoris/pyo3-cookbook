use chrono::NaiveDate;
use rust_decimal::Decimal;
//use std::collections::HashMap;
//use serde_json::Value;

//type JsonMap = HashMap<String, Value>;

struct Conta {
    id_conta: i64,
    nome: String,
    codigo: String,
    permite_lancamentos: bool,
}

struct Veiculo {
    id_veiculo: i64,
    nome: String,
}

struct RelacionamentoContaHierarquia {
    id_rel_conta_hierarquia: i64,
    id_hierarquia: i64,
    id_parent: i64,
    id_child: i64,
}

struct HierarquiaContas {
    id_hierarquia: i64,
    nome: String,
    descricao: Option<String>,
}

struct Lancamento {
    id_lancamento: i64,
    id_veiculo: i64,
    id_conta: i64,
    data: NaiveDate,
    valor: Decimal,
    //meta: Option<Value>,
    meta: String,
}
