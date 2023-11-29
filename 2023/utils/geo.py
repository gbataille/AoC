from copy import deepcopy
from dataclasses import dataclass
from typing import Callable, Dict, Generator, Generic, Hashable, List, Optional, Set, Tuple, TypeVar


@dataclass(frozen=True)
class Point2D:
    x: int
    y: int


@dataclass(frozen=True)
class Point3D:
    x: int
    y: int
    z: int


C = TypeVar('C')
T = TypeVar('T', bound=Hashable)


@dataclass
class Cell(Generic[C, T]):
    coordinates: C
    value: T

    def __hash__(self):
        return hash(self.coordinates) | hash(self.value)

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Cell):
            return False
        return self.coordinates == other.coordinates and self.value == other.value


@dataclass
class Array2D(Generic[T]):
    cells: List[List[Cell[Point2D, T]]]
    height: int
    width: int

    def __init__(
        self,
        cells: List[List[T]],
    ):
        self.cells = [[Cell(Point2D(cell_idx, line_idx), cell_value) for cell_idx, cell_value in enumerate(line)]
                      for line_idx, line in enumerate(cells)]
        self.width = len(cells[0])
        self.height = len(cells)

    def __iter__(self) -> Generator[Cell[Point2D, T], None, None]:
        return self.step_through()

    def get_value_at(self, x: int, y: int) -> T:
        return self.get_cell_at(x, y).value

    def get_cell_at(self, x: int, y: int) -> Cell[Point2D, T]:
        return self.cells[y][x]

    def set_value_at(self, x: int, y: int, value: T) -> 'Array2D':
        if x > self.width or x < 0:
            raise IndexError(f'x {x} is outside the array boundaries')
        if y > self.height or y < 0:
            raise IndexError(f'y {y} is outside the array boundaries')

        return self.set_cell(Cell(Point2D(x, y), value))

    def set_value_at_point(self, p: Point2D, value: T) -> 'Array2D':
        return self.set_cell(Cell(p, value))

    def set_cell(self, cell: Cell[Point2D, T]) -> 'Array2D':
        self.cells[cell.coordinates.y][cell.coordinates.x] = cell
        return self

    def step_through(self, horizontally: bool = True) -> Generator[Cell[Point2D, T], None, None]:
        if horizontally:
            for y in range(self.height):
                for x in range(self.width):
                    yield self.get_cell_at(x, y)
        else:
            for x in range(self.width):
                for y in range(self.height):
                    yield self.get_cell_at(x, y)

    def neighbours(self, x: int, y: int, include_diags: bool = False) -> Generator[Cell[Point2D, T], None, None]:
        for dx in range(-1, 2):
            for dy in range(-1, 2):
                # skip diag
                if not include_diags and dx != 0 and dy != 0:
                    continue

                # skip self
                if dx == 0 and dy == 0:
                    continue

                # Handle borders
                if not 0 <= x + dx < self.width:
                    continue
                if not 0 <= y + dy < self.height:
                    continue

                yield self.get_cell_at(x + dx, y + dy)

    @staticmethod
    def from_string(
        input_str: str,
        str_elem_to_T_elem: Callable[[str], T],
        line_separator: str = '\n',
        elem_separator: Optional[str] = None,
    ) -> 'Array2D[T]':
        cells: List[List[T]] = []
        for line_str in input_str.strip().split(line_separator):
            line = []

            line_list = []
            if elem_separator:
                line_list = line_str.strip().split(elem_separator)
            else:
                line_list = list(line_str.strip())

            for elem in line_list:
                line.append(str_elem_to_T_elem(elem))

            cells.append(line)

        return Array2D(cells)

    def to_string(
        self,
        elem_to_str: Callable[[T], str] = str,
        line_separator: str = '\n',
        elem_separator: str = '',
    ):
        out = ''
        for line in self.cells:
            line_str = ''
            for elem in line:
                line_str += elem_separator + elem_to_str(elem.value)

            if elem_separator != '':
                line_str = line_str[1:]

            out += line_str + line_separator

        return out

    @staticmethod
    def empty(width: int, height: int, elem: T) -> 'Array2D':
        cells = []

        line = []
        for _ in range(width):
            line.append(elem)

        for _ in range(height):
            cells.append(deepcopy(line))

        return Array2D(cells)


def dijkstra(weight_map: Array2D[int], start: Point2D, end: Point2D) -> int:
    INFINITY = -1
    path_weight: Array2D[int] = Array2D.empty(weight_map.width, weight_map.height, INFINITY)
    unvisited: Set[Point2D] = set(map(lambda x: x.coordinates, weight_map.step_through()))

    # Accelerating structure
    distance_to_node: Dict[int, List[Point2D]] = {0: [Point2D(0, 0)]}

    def smallest_head_node() -> Tuple[Point2D, int]:
        distances = list(distance_to_node.keys())
        distances.sort()
        return distance_to_node[distances[0]][0], distances[0]

        min_weight = -1
        shortest_head = Point2D(-1, -1)

        for node in unvisited:
            value = path_weight.get_value_at(node.x, node.y)
            if value == -1:
                continue

            if min_weight == -1 or value < min_weight:
                min_weight = value
                shortest_head = node

        return shortest_head, min_weight

    def done() -> bool:
        return path_weight.get_value_at(end.x, end.y) != INFINITY or len(unvisited) == 0

    path_weight.set_value_at_point(start, 0)

    i = 0
    node = start
    current_weight = 0
    from datetime import datetime
    while not done():
        # if i % 100 == 0:
        #     print(
        #         path_weight.to_string(
        #             lambda x: '  .' if x == -1 else strings.replace_leading_x('0', ' ',
        #                                                                       str(x).zfill(3)),
        #             elem_separator=' ',
        #         ))
        #     input("press enter\n")
        beginning = datetime.now().timestamp() * 1000
        for neighbour in weight_map.neighbours(node.x, node.y):
            neighbour_weight = path_weight.get_value_at(neighbour.coordinates.x, neighbour.coordinates.y)
            new_weight = current_weight + neighbour.value
            if neighbour_weight == INFINITY or new_weight < neighbour_weight:
                path_weight.set_value_at_point(neighbour.coordinates, new_weight)

                if neighbour_weight != INFINITY:
                    distance_to_node[neighbour_weight].remove(neighbour.coordinates)
                    if not distance_to_node[neighbour_weight]:
                        del distance_to_node[neighbour_weight]
                if distance_to_node.get(new_weight) is None:
                    distance_to_node[new_weight] = []
                distance_to_node[new_weight].append(neighbour.coordinates)

        distance_to_node[current_weight].remove(node)
        if not distance_to_node[current_weight]:
            del distance_to_node[current_weight]
        unvisited.remove(node)
        node, current_weight = smallest_head_node()
        i += 1
        print(datetime.now().timestamp() * 1000 - beginning)

    return path_weight.get_value_at(end.x, end.y)
