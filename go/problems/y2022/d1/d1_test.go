package day1

import (
	"fmt"
  "os"
  "path"
	"testing"

	"github.com/pkg/errors"
	"github.com/stretchr/testify/require"
	"go.uber.org/zap"
)

func Test_SolvePart1(t *testing.T) {
	// LogLevel.SetLevel(zap.DebugLevel)

  curDir, err := os.Getwd()
  require.NoError(t, err)
  bInput, err := os.ReadFile(path.Join(curDir, "input.txt"))
  require.NoError(t, err)
  input := string(bInput)
  res, err := Part1(input)
  require.NoError(t, err)
  printResult(res)
}

func Test_SolvePart2(t *testing.T) {
	// LogLevel.SetLevel(zap.DebugLevel)

  curDir, err := os.Getwd()
  require.NoError(t, err)
  bInput, err := os.ReadFile(path.Join(curDir, "input.txt"))
  require.NoError(t, err)
  input := string(bInput)
  res, err := Part2(input)
  require.NoError(t, err)
  printResult(res)
}

func Test_RunPart1(t *testing.T) {
	// LogLevel.SetLevel(zap.DebugLevel)

  input := `
`
  res, err := Part1(input)
  require.NoError(t, err)
  printResult(res)
}

func Test_RunPart2(t *testing.T) {
	// LogLevel.SetLevel(zap.DebugLevel)

  input := `
`
  res, err := Part2(input)
  require.NoError(t, err)
  printResult(res)
}

func Test_TableTest(t *testing.T) {
	LogLevel.SetLevel(zap.InfoLevel)
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

func printResult(res string) {
  fmt.Println()
  fmt.Println()
  fmt.Println("######################")
  fmt.Println("Result:")
  fmt.Println()
  fmt.Println(res)
  fmt.Println()
  fmt.Println("######################")
  fmt.Println()
  fmt.Println()
}
