
import purepython
import pandas as pd

def test_module():
    print(f"purepython sum: {purepython.pure_python_sum(2, 3)}")
    print(f"native triple: {purepython._core.triple(3)}")

def test_dataframe():
    df = pd.DataFrame({'scores': [10.0, 20.0, 30.0], 'other': [2.0, 3.0, 2.0]})
    purepython.python_axpy(df, 10.0, 'scores', 'other')