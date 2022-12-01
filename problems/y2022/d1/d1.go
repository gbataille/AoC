package day1

import (
	"os"
	"strconv"
	"strings"

	"go.uber.org/zap"
	"golang.org/x/exp/slices"
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
	logger.Info("Day 1")
}

// Part1 solves the Part 1 of the problem
func Part1(inputData string) (string, error) {
	var res string
	logger.Info("Running day 1 Part 1")

	type Elf struct {
		calories uint64
	}

	curElf := &Elf{}
	var elves []*Elf
	max := uint64(0)
	for _, line := range strings.Split(inputData, "\n") {
		if line == "" {
			elves = append(elves, curElf)
			if curElf.calories > max {
				max = curElf.calories
			}
			curElf = &Elf{}
			continue
		}

		cal, _ := strconv.ParseUint(line, 10, 64)
		curElf.calories += cal
	}

	res = strconv.FormatUint(max, 10)
	return res, nil
}

// Part2 solves the Part 2 of the problem
func Part2(inputData string) (string, error) {
	var res string
	logger.Info("Running day 1 Part 1")

	type Elf struct {
		calories uint64
	}

	curElf := &Elf{}
	var elves []*Elf
	max := uint64(0)
	for _, line := range strings.Split(inputData, "\n") {
		if line == "" {
			elves = append(elves, curElf)
			if curElf.calories > max {
				max = curElf.calories
			}
			curElf = &Elf{}
			continue
		}

		cal, _ := strconv.ParseUint(line, 10, 64)
		curElf.calories += cal
	}

	slices.SortFunc(elves, func(a *Elf, b *Elf) bool { return a.calories > b.calories })

	tot := uint64(0)
	for i := 0; i < 3; i++ {
		tot += elves[i].calories
	}

	res = strconv.FormatUint(tot, 10)
	return res, nil
}
