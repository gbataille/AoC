package config

import (
	"fmt"

	"github.com/spf13/viper"
)

const (
	yearKey          = "year"
	aocRootUrlKey    = "rootUrl"
	aocSessionKeyKey = "sessionKey"
)

type configData struct {
	Year          uint64
	AocRootUrl    string
	AocSessionKey string
}

var cfg configData

func Year() uint64 {
	return cfg.Year
}

func AocUrl() string {
	return cfg.AocRootUrl
}

func AocSessionKey() string {
	return cfg.AocSessionKey
}

func Setup() {
	yearCfg := viper.GetInt64(yearKey)
	if yearCfg == 0 {
		panic(fmt.Sprintf("invalid or missing %v in config, should be an int", yearKey))
	}
	cfg.Year = uint64(yearCfg)

	aocRootUrl := viper.GetString(aocRootUrlKey)
	if aocRootUrl == "" {
		panic(fmt.Sprintf("invalid or missing %v in config, should be a string", aocRootUrlKey))
	}
	cfg.AocRootUrl = aocRootUrl

	aocSessionKey := viper.GetString(aocSessionKeyKey)
	if aocSessionKey == "" {
		panic(fmt.Sprintf("invalid or missing %v in config, should be a string", aocSessionKeyKey))
	}
	cfg.AocSessionKey = aocSessionKey
}
