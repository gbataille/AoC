import os


def debug(string: str) -> None:
    if os.getenv('AOC_TEST') == 'true':
        print(string)
