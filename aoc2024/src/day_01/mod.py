from common.src.util import print_green, read_input


def part_1(input: str) -> int:
    lines = input.rstrip().split("\n")

    left_numbers = [int(line.split()[0]) for line in lines]
    right_numbers = [int(line.split()[1]) for line in lines]

    left_numbers = sorted(left_numbers)
    right_numbers = sorted(right_numbers)

    sum: int = 0
    for left, right in zip(left_numbers, right_numbers):
        sum += abs(left - right)

    return sum


def part_2(input: str) -> int:

    lines = input.rstrip().split("\n")

    left_numbers = [int(line.split()[0]) for line in lines]
    hashmap = {int(line.split()[0]): 0 for line in lines}
    [
        hashmap.update({int(line.split()[1]): hashmap[int(line.split()[1])] + 1})
        for line in lines
        if int(line.split()[1]) in hashmap
    ]

    sum: int = 0
    for key, value in hashmap.items():
        sum += (key * value) * left_numbers.count(key)

    return sum


def test_part_1():
    input = """
3   4
4   3
2   5
1   3
3   9
3   3
""".lstrip()
    assert part_1(input) == 11


def test_run_part_1():
    result = part_1(read_input(__file__))
    print_green(f"Day 1, part 1 result: {result}")
    assert result == 1189304


def test_part_2():
    input = """
3   4
4   3
2   5
1   3
3   9
3   3
""".lstrip()
    assert part_2(input) == 31


def test_run_part_2():
    result = part_2(read_input(__file__))
    print_green(f"Day 1, part_2 result: {result}")
    assert result == 23927637
