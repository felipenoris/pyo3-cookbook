from os import path
import purepython
import pandas as pd

def test_veiculo():
    v = purepython.Veiculo(10, "Emp10")
    print(f"id_veiculo = {v.id_veiculo}, nome = {v.nome}")
    v.nome = "other Emp"
    print(f"id_veiculo = {v.id_veiculo}, nome = {v.nome}")

def test_load_xl():
    SCRIPT_DIR = path.dirname(path.abspath(__file__))
    fp = path.join(SCRIPT_DIR, "lancamentos.xlsx")
    assert path.isfile(fp)
    xl_df_lancamentos = pd.read_excel(fp, "Lançamentos")
    print(xl_df_lancamentos)