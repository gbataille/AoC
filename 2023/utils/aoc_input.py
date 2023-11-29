from pathlib import Path

import requests


def get_input(day_num: int) -> Path:
    """
    Test some comments
    """
    INPUT_URL = f'https://adventofcode.com/2023/day/{day_num}/input'
    SESSION = '53616c7465645f5f2e9d281a3453a5764870b5c3e64e06b30adc04a066e5945453b3844a15424c3f576254c03fb5b93e0e85dbcbc1e2989990e17f7462b2033c'  # noqa
    input_file_path = Path(__file__).parent.parent.joinpath(
        f'day{str(day_num)}', 'input.txt')
    if not input_file_path.exists():
        input_data = requests.get(INPUT_URL, cookies={'session': SESSION}).text
        input_file_path.write_text(input_data)
    return input_file_path
