# TODO
from rurp import sum_as_string  # type: ignore[attr-defined]


def test_123() -> None:
    assert sum_as_string(1, 2) == "3"
