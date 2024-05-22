import calculator  # type: ignore


def test_add() -> None:
    assert calculator.add(1, 2) == 3.0

def test_subtract() -> None:
    assert calculator.subtract(3, 1) == 2.0

def test_multiply() -> None:
    assert calculator.multiply(2, 3) == 6.0

def test_divide() -> None:
    assert calculator.divide(6, 2) == 3.0
    try:
        calculator.divide(1, 0)
    except ValueError:
        assert True
    else:
        assert False
