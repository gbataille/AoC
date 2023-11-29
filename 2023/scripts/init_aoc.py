from pathlib import Path

day_file = """from pathlib import Path
from typing import Optional

from utils import aoc_input
from utils.out import debug


TEST_DATA = ""


def main_part_1(input_file: Optional[Path]) -> None:
    input_data = TEST_DATA
    if input_file is not None:
        input_data = input_file.read_text()


def main_part_2(input_file: Optional[Path] = None) -> None:
    input_data = TEST_DATA
    if input_file:
        input_data = input_file.read_text()


def main(day_num: int, part_num: int, with_test_data=True) -> None:
    input_file = None
    if not with_test_data:
        input_file = aoc_input.get_input(day_num)

    if part_num == 1:
        main_part_1(input_file)
    elif part_num == 2:
        main_part_2(input_file)
    else:
        raise ValueError(f"Invalid part number {part_num}")"""


def main():
    root_path = Path(__file__).parent.parent
    for day in range(1, 26):
        day_path = root_path.joinpath(f'day{str(day)}')
        if not day_path.exists():
            day_path.mkdir()
        day_path.joinpath("__init__.py").write_text("from .main import *")
        day_path.joinpath("main.py").write_text(day_file)


if __name__ == "__main__":
    main()
