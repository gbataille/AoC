package matrixutils

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	"testing"

	"github.com/gbataille/AoC_2022/internal/logging"
	"github.com/stretchr/testify/require"
)

func TestMain(m *testing.M) {
	// Disable debug level logs for tests
	logging.LogLevel = "info"
	logging.Setup()

	os.Exit(m.Run())
}

func TestSplitNewline(t *testing.T) {
	in := `foo
bar
baz`
	out := strings.Split(in, "\n")
	fmt.Printf("%v\n", out[0])
	fmt.Printf("%v\n", out[1])
	fmt.Printf("%v\n", out[2])
	fmt.Printf("%v\n", out)

}

func TestMatrix2D_AddLine(t *testing.T) {
	tests := []struct {
		name           string
		inputMatrix    *Matrix2D[uint64]
		inputLine      []uint64
		expectedOutput *Matrix2D[uint64]
		expectedError  error
	}{
		{name: "empty matrix, empty line -> error", inputMatrix: &Matrix2D[uint64]{}, inputLine: []uint64{}, expectedOutput: nil, expectedError: UnsupportedError},
		{name: "empty matrix, valid line", inputMatrix: &Matrix2D[uint64]{}, inputLine: []uint64{12, 42}, expectedOutput: &Matrix2D[uint64]{height: 1, width: 2, values: [][]uint64{{12, 42}}}, expectedError: nil},
		{name: "non empty matrix, non matching line -> error", inputMatrix: &Matrix2D[uint64]{height: 1, width: 1, values: [][]uint64{{33}}}, inputLine: []uint64{12, 42}, expectedOutput: nil, expectedError: UnsupportedError},
		{name: "non empty matrix, valid line -> added at the end", inputMatrix: &Matrix2D[uint64]{height: 1, width: 2, values: [][]uint64{{33, 666}}}, inputLine: []uint64{12, 42}, expectedOutput: &Matrix2D[uint64]{height: 2, width: 2, values: [][]uint64{{33, 666}, {12, 42}}}, expectedError: nil},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			res, err := test.inputMatrix.AddLine(test.inputLine)
			if test.expectedError != nil {
				require.Error(t, err)
				require.ErrorIs(t, err, test.expectedError)
				require.Nil(t, res)
			} else {
				require.NoError(t, err)
				require.NotNil(t, res)
				require.Equal(t, test.expectedOutput, res)
			}
		})
	}
}

func TestMatrix2D_Get(t *testing.T) {
	tests := []struct {
		name           string
		inputMatrix    *Matrix2D[uint64]
		inputX         uint64
		inputY         uint64
		expectedOutput *Cell2D[uint64]
		expectedError  error
	}{
		{name: "x out of bound -> error", inputMatrix: &Matrix2D[uint64]{height: 1, width: 1, values: [][]uint64{{12}}}, inputX: 1, inputY: 0, expectedOutput: nil, expectedError: UnsupportedError},
		{name: "y out of bound -> error", inputMatrix: &Matrix2D[uint64]{height: 1, width: 1, values: [][]uint64{{12}}}, inputX: 0, inputY: 1, expectedOutput: nil, expectedError: UnsupportedError},
		{name: "valid 0,0", inputMatrix: &Matrix2D[uint64]{height: 2, width: 2, values: [][]uint64{{12, 13}, {22, 33}}}, inputX: 0, inputY: 0, expectedOutput: &Cell2D[uint64]{x: 0, y: 0, value: 12}, expectedError: nil},
		{name: "valid 1,0", inputMatrix: &Matrix2D[uint64]{height: 2, width: 2, values: [][]uint64{{12, 13}, {22, 33}}}, inputX: 1, inputY: 0, expectedOutput: &Cell2D[uint64]{x: 1, y: 0, value: 13}, expectedError: nil},
		{name: "valid 0,1", inputMatrix: &Matrix2D[uint64]{height: 2, width: 2, values: [][]uint64{{12, 13}, {22, 33}}}, inputX: 0, inputY: 1, expectedOutput: &Cell2D[uint64]{x: 0, y: 1, value: 22}, expectedError: nil},
		{name: "valid 1,1", inputMatrix: &Matrix2D[uint64]{height: 2, width: 2, values: [][]uint64{{12, 13}, {22, 33}}}, inputX: 1, inputY: 1, expectedOutput: &Cell2D[uint64]{x: 1, y: 1, value: 33}, expectedError: nil},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			res, err := test.inputMatrix.Get(test.inputX, test.inputY)
			if test.expectedError != nil {
				require.Error(t, err)
				require.ErrorIs(t, err, test.expectedError)
				require.Nil(t, res)
			} else {
				require.NoError(t, err)
				require.NotNil(t, res)
				require.Equal(t, test.expectedOutput, res)
			}
		})
	}
}

func TestMatrix2D_Matrix2DFromString(t *testing.T) {
	multilineInput := `1 2 3
4 5 6
7 8 9
11 12 13`

	tests := []struct {
		name           string
		inputString    string
		lineSep        string
		colSep         string
		expectedOutput *Matrix2D[uint64]
		expectedError  error
	}{
		{name: "bad type input", inputString: "1 1.3", lineSep: "\n", colSep: " ", expectedOutput: nil, expectedError: strconv.ErrSyntax},
		{name: "valid input", inputString: multilineInput, lineSep: "\n", colSep: " ", expectedOutput: &Matrix2D[uint64]{height: 4, width: 3, values: [][]uint64{{1, 2, 3}, {4, 5, 6}, {7, 8, 9}, {11, 12, 13}}}, expectedError: nil},
		{name: "valid input, non standard separators", inputString: "1,2,3;4,5,6", lineSep: ";", colSep: ",", expectedOutput: &Matrix2D[uint64]{height: 2, width: 3, values: [][]uint64{{1, 2, 3}, {4, 5, 6}}}, expectedError: nil},
		{name: "invalid input", inputString: "1,2,3;4,5,6,7", lineSep: ";", colSep: ",", expectedOutput: nil, expectedError: UnsupportedError},
	}

	for _, test := range tests {
		converter := func(in string) (uint64, error) {
			out, err := strconv.ParseUint(in, 10, 64)
			if err != nil {
				return 0, err
			}
			return out, nil
		}

		t.Run(test.name, func(t *testing.T) {
			res, err := Matrix2DFromString(test.inputString, test.lineSep, test.colSep, converter)
			if test.expectedError != nil {
				require.Error(t, err)
				require.ErrorIs(t, err, test.expectedError)
				require.Nil(t, res)
			} else {
				require.NoError(t, err)
				require.NotNil(t, res)
				require.Equal(t, test.expectedOutput, res)
			}
		})
	}
}
