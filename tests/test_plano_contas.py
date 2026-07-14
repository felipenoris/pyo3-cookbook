
import purepython

def test_veiculo():
    v = purepython.Veiculo(10, "Emp10")
    print(f"id_veiculo = {v.id_veiculo}, nome = {v.nome}")
    v.nome = "other Emp"
    print(f"id_veiculo = {v.id_veiculo}, nome = {v.nome}")