
import purepython

def test_module():
    print(f"purepython sum: {purepython.pure_python_sum(2, 3)}")
    print(f"native triple: {purepython._core.triple(3)}")