package day5

import (
	"fmt"
	"testing"

	"github.com/pkg/errors"
	"github.com/stretchr/testify/require"
)

func Test_SolvePart1(t *testing.T) {
  input := `
`
  res, err := Part1(input)
  require.NoError(t, err)
  fmt.Println(res)
}

func Test_SolvePart2(t *testing.T) {
  input := `
`
  res, err := Part2(input)
  require.NoError(t, err)
  fmt.Println(res)
}

func Test_RunPart1(t *testing.T) {
  input := `
`
  res, err := Part1(input)
  require.NoError(t, err)
  fmt.Println(res)
}

func Test_RunPart2(t *testing.T) {
  input := `
`
  res, err := Part2(input)
  require.NoError(t, err)
  fmt.Println(res)
}

func Test_TableTest(t *testing.T) {
  tests := []struct{
    PartNumber int
    InputData string
    ExpectedOutput string
  }{
  }

  for _, test := range tests {
    t.Run(test.InputData[:15], func(t *testing.T) {
      var result string
      var err error

      switch test.PartNumber {
      case 1:
        result, err = Part1(test.InputData)
      case 2:
        result, err = Part2(test.InputData)
      default:
        err = errors.Errorf("invalid PartNumber. Must be 1 or 2")
      }

      require.NoError(t, err)
      require.Equal(t, test.ExpectedOutput, result)
    })
  }
}
