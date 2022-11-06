package matrixutils

import (
	"strings"

	"github.com/gbataille/AoC_2022/internal/logging"
	"github.com/pkg/errors"
)

var UnsupportedError = errors.Errorf("unsupported operation")

type Cell2D[T any] struct {
	x     uint64
	y     uint64
	value T
}

type Matrix2D[T any] struct {
	height uint64
	width  uint64
	values [][]T
}

func (m *Matrix2D[T]) AddLine(line []T) (*Matrix2D[T], error) {
	if len(line) == 0 {
		return nil, errors.Wrapf(UnsupportedError, "empty lines not supported")
	}

	if len(m.values) == 0 {
		m.width = uint64(len(line))
	} else {
		if len(m.values[0]) != len(line) {
			return nil, errors.Wrapf(UnsupportedError, "All lines must have the same size %v", len(m.values[0]))
		}
	}

	m.values = append(m.values, line)
	m.height += 1

	return m, nil
}

func (m *Matrix2D[T]) Get(x, y uint64) (*Cell2D[T], error) {
	if x > m.width-1 {
		return nil, errors.Wrapf(UnsupportedError, "x must be a valid matrix coordinate")
	}
	if y > m.height-1 {
		return nil, errors.Wrapf(UnsupportedError, "y must be a valid matrix coordinate")
	}

	return &Cell2D[T]{x: x, y: y, value: m.values[y][x]}, nil
}

func Matrix2DFromString[T any](stringMatrix, lineSeparator, columnSeparator string, toValue func(string) (T, error)) (*Matrix2D[T], error) {
	m2D := &Matrix2D[T]{}

	for lineIdx, stringLine := range strings.Split(stringMatrix, lineSeparator) {
		var line []T

		for colIdx, stringValue := range strings.Split(stringLine, columnSeparator) {
			logging.Logger.Debugf("Processing line %v column %v: value", lineIdx, colIdx)

			value, err := toValue(stringValue)
			if err != nil {
				return nil, err
			}

			line = append(line, value)
		}

		_, err := m2D.AddLine(line)
		if err != nil {
			return nil, errors.Wrapf(UnsupportedError, "unable to add line #%v (%v) to matrix", lineIdx, stringLine)
		}
	}

	return m2D, nil
}
