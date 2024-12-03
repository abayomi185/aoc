from common.src.util import read_input


def part_1(input: str) -> int:
    return 1


def part_2(input: str) -> int:
    return 2


def test_part_1():
    input = """
""".lstrip()
    assert part_1(input) == 1


def test_run_part_1():
    result = part_1(read_input(__file__))
    assert result == 1


def test_part_2():
    input = """
""".lstrip()
    assert part_2(input) == 2


def test_run_part_2():
    result = part_2(read_input(__file__))
    assert result == 2
