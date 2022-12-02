package day2

import (
	"fmt"
	"os"
	"strconv"
	"strings"

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
	logger.Info("Day 2")
}

type RPS string

const (
	Rock     RPS = "Rock"
	Paper    RPS = "Paper"
	Scissors RPS = "Scissors"
)

func decode(play string) RPS {
	switch play {
	case "A":
		return Rock
	case "B":
		return Paper
	case "C":
		return Scissors
	case "X":
		return Rock
	case "Y":
		return Paper
	case "Z":
		return Scissors
	default:
		panic(fmt.Sprintf("WTF %v\n", play))
	}
}

func score(me RPS, other RPS) int {
	var score int
	switch me {
	case Rock:
		score = 1
		switch other {
		case Rock:
			return score + 3
		case Paper:
			return score
		case Scissors:
			return score + 6
		}
	case Paper:
		score = 2
		switch other {
		case Rock:
			return score + 6
		case Paper:
			return score + 3
		case Scissors:
			return score
		}
	case Scissors:
		score = 3
		switch other {
		case Rock:
			return score
		case Paper:
			return score + 6
		case Scissors:
			return score + 3
		}
	}
	panic(fmt.Sprintf("WTF %v %v\n", me, other))
}

func computePlay(other RPS, instr string) RPS {
	switch other {
	case Rock:
		switch instr {
		case "X":
			return Scissors
		case "Y":
			return Rock
		case "Z":
			return Paper
		}
	case Paper:
		switch instr {
		case "X":
			return Rock
		case "Y":
			return Paper
		case "Z":
			return Scissors
		}
	case Scissors:
		switch instr {
		case "X":
			return Paper
		case "Y":
			return Scissors
		case "Z":
			return Rock
		}
	}
	panic(fmt.Sprintf("WTF %v, %v", other, instr))
}

// Part1 solves the Part 1 of the problem
func Part1(inputData string) (string, error) {
	var res string
	logger.Info("Running day 2 Part 1")

	var total int
	rounds := strings.Split(inputData, "\n")
	for _, round := range rounds {
		if round == "" {
			continue
		}
		played := strings.Split(round, " ")
		other := played[0]
		me := played[1]
		total += score(decode(me), decode(other))
	}

	res = strconv.FormatInt(int64(total), 10)
	return res, nil
}

// Part2 solves the Part 2 of the problem
func Part2(inputData string) (string, error) {
	var res string
	logger.Info("Running day 2 Part 1")

	var total int
	rounds := strings.Split(inputData, "\n")
	for _, round := range rounds {
		if round == "" {
			continue
		}
		played := strings.Split(round, " ")
		other := played[0]
		me := played[1]
		total += score(computePlay(decode(other), me), decode(other))
	}

	res = strconv.FormatInt(int64(total), 10)
	return res, nil
}
