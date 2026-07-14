
from ._core import triple
from ._core import axpy
from ._core import Veiculo
from ._core import process_arrow_batch
from ._core import process_arrow_batch_mult2

def pure_python_sum(a, b):
    return a + b

def python_axpy(df, a, col_name_x, col_name_y):
    x = df[col_name_x].to_numpy(copy=False)
    y = df[col_name_y].to_numpy(copy=False)

    print(type(x))

    result = axpy(a, x, y)
    print(result)