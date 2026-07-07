
import purepython

def main():
    print("Hello from pyo3-cookbook!")
    print(f"purepython sum {purepython.pure_python_sum(1, 2)}")
    print(f"rust tripe: {purepython._core.triple(3)}")


if __name__ == "__main__":
    main()
