from pathlib import Path


def print_green(message: str):
    print("\x1b[32m{}\x1b[0m", message)


def read_input(module_pathname: str) -> str:
    path = Path(module_pathname).parent / "input.txt"
    return path.read_text()
