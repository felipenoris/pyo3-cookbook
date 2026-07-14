from os import path
import purepython
import pandas as pd
import pyarrow as pa

def test_veiculo():
    v = purepython.Veiculo(10, "Emp10")
    print(f"id_veiculo = {v.id_veiculo}, nome = {v.nome}")
    v.nome = "other Emp"
    print(f"id_veiculo = {v.id_veiculo}, nome = {v.nome}")

def test_load_xl():
    SCRIPT_DIR = path.dirname(path.abspath(__file__))
    fp = path.join(SCRIPT_DIR, "lancamentos.xlsx")
    assert path.isfile(fp)

    xl_df_lancamentos = pd.read_excel(fp,
        sheet_name = "Lançamentos",
        dtype_backend="pyarrow",
    )

    # Tratamento dos dados
    xl_df_lancamentos = xl_df_lancamentos.astype({
        "Veículo": "string[pyarrow]",
        "Código": "string[pyarrow]",
        "Valor": "float64[pyarrow]",
    })

    xl_df_lancamentos["Data"] = pd.to_datetime(xl_df_lancamentos["Data"])

    #Veículo	Código	Data	Valor

    print(xl_df_lancamentos)
    print()
    print(xl_df_lancamentos["Veículo"])
    print(xl_df_lancamentos["Código"])
    print(xl_df_lancamentos["Data"])
    print(xl_df_lancamentos["Valor"])

    # Schema Arrow
    schema = pa.schema([
        pa.field("Veículo", pa.string(), nullable=False),
        pa.field("Código", pa.string(), nullable=False),
        pa.field("Data", pa.date32(), nullable=False),
        pa.field("Valor", pa.float64(), nullable=False),
    ])

    table = pa.Table.from_pandas(xl_df_lancamentos, schema=schema, preserve_index=False)

    result_batches = list()

    for batch in table.to_batches():
        # pyo3-arrow retorna estruturas do pacote arro3 do python, implementação minimalista do arrow
        # pyarrow é a implementação padrão
        result = pa.record_batch(purepython.process_arrow_batch_mult2(batch, "Valor"))
        print(type(result))
        result_batches.append(result)

    nova_table = pa.Table.from_batches(result_batches)

    print()
    print("nova table")
    print(nova_table.to_pandas())

def test_inline_arrow_table():
    names = pa.array(["Alice", "Bob", "Charlie", "David"])
    ages = pa.array([25, 30, 35, 40], type=pa.int32())
    salaries = pa.array([50000.0, 60000.0, None, 80000.0])  # Supports null out of the box

    # Combine arrays into a structured PyArrow Table
    data = {"name": names, "age": ages, "salary": salaries}
    table = pa.table(data)

    for batch in table.to_batches():
        purepython.process_arrow_batch(batch)

def test_schema_arrow():
    # 1. Define the schema
    schema = pa.schema([
        ('id', pa.int32()),
        ('values', pa.int32())
    ])

    # 2. Create sample data arrays
    ids = pa.array([1, 2, 3], type=pa.int32())
    values = pa.array([10, 20, 30], type=pa.int32())

    # 3. Build the RecordBatch
    batch = pa.RecordBatch.from_arrays([ids, values], schema=schema)
    purepython.process_arrow_batch(batch)