package logging

import (
	"go.uber.org/zap"
)

var LogLevel string
var Logger *zap.SugaredLogger

func init() {
	// avoid NPE by always having a logger initialized
	devLog, err := zap.NewDevelopment()
	if err != nil {
		panic(err.Error())
	}

	Logger = devLog.Sugar()
}

func Setup() {
	level, err := zap.ParseAtomicLevel(LogLevel)
	if err != nil {
		panic(err.Error())
	}

	cfg := zap.Config{
		Level:            level,
		Development:      true,
		Encoding:         "console",
		EncoderConfig:    zap.NewDevelopmentEncoderConfig(),
		OutputPaths:      []string{"stdout"},
		ErrorOutputPaths: []string{"stderr"},
	}
	logger, err := cfg.Build()
	if err != nil {
		panic(err.Error())
	}

	Logger = logger.Sugar()
}
