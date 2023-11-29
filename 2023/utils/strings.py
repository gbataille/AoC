import re


def replace_leading_x(x: str, y: str, original: str) -> 'str':
    regex_str = f'({x}+).*'
    matches = re.match(regex_str, original)
    if matches:
        nb_char = len(matches[1])
        return y * nb_char + original[nb_char:]

    return original
