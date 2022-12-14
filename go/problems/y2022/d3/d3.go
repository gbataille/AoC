package day3

import (
	"os"

	"go.uber.org/zap"
)

var logger *zap.SugaredLogger
var LogLevel *zap.AtomicLevel

func init() {
	levelStr := os.Getenv("LOGLEVEL")
	if levelStr == "" {
		levelStr = "info"
	}
	level, err := zap.ParseAtomicLevel(levelStr)
	if err != nil {
		panic(err.Error())
	}
	LogLevel = &level

	cfg := zap.Config{
		Level:            level,
		Development:      true,
		Encoding:         "console",
		EncoderConfig:    zap.NewDevelopmentEncoderConfig(),
		OutputPaths:      []string{"stdout"},
		ErrorOutputPaths: []string{"stderr"},
	}
	rawLogger, err := cfg.Build()
	if err != nil {
		panic(err.Error())
	}

	logger = rawLogger.Sugar()
}


func main() {
  logger.Info("Day 3")
}

// Part1 solves the Part 1 of the problem
func Part1(inputData string) (string, error) {
  var res string
  logger.Info("Running day 3 Part 1")

  return res, nil
}

// Part2 solves the Part 2 of the problem
func Part2(inputData string) (string, error) {
  var res string
  logger.Info("Running day 3 Part 2")

  return res, nil
}
