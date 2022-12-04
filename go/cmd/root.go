/*
Copyright © 2022 Grégory Bataille gregory.bataille@gmail.com
*/
package cmd

import (
	"os"

	"github.com/gbataille/AoC_2022/internal/config"
	"github.com/gbataille/AoC_2022/internal/logging"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

var cfgFile string

// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
	Use:   "AoC_2022",
	Short: "Interact with the AoC 2022 problems and solutions",
	Long:  "",
	// Uncomment the following line if your bare application
	// has an action associated with it:
	// Run: func(cmd *cobra.Command, args []string) { },
}

// Execute adds all child commands to the root command and sets flags appropriately.
// This is called by main.main(). It only needs to happen once to the rootCmd.
func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	cobra.OnInitialize(initConfig)

	// Here you will define your flags and configuration settings.
	// Cobra supports persistent flags, which, if defined here,
	// will be global for your application.

	rootCmd.PersistentFlags().StringVar(&cfgFile, "config", "", "config file (default is $HOME/.AoC_2022.yaml)")
	envLogLevel := os.Getenv("LOGLEVEL")
	defaultLogLevel := envLogLevel
	if defaultLogLevel == "" {
		defaultLogLevel = "info"
	}
	rootCmd.PersistentFlags().StringVar(&logging.LogLevel, "logLevel", defaultLogLevel, "log level")

	// Cobra also supports local flags, which will only run
	// when this action is called directly.
	rootCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}

// initConfig reads in config file and ENV variables if set.
func initConfig() {
	if cfgFile != "" {
		viper.SetConfigFile(cfgFile)
	} else {
		curDir, err := os.Getwd()
		cobra.CheckErr(err)
		home, err := os.UserHomeDir()
		cobra.CheckErr(err)

		viper.AddConfigPath(curDir)
		viper.AddConfigPath(curDir + "/..")
		viper.AddConfigPath(home)
		viper.SetConfigType("yaml")
		viper.SetConfigName("AoC.yml")
	}

	viper.AutomaticEnv() // read in environment variables that match

	// If a config file is found, read it in.
	if err := viper.ReadInConfig(); err == nil {
		logging.Logger.Debugln("Using config file:", viper.ConfigFileUsed())
	}

	config.Setup()
	logging.Setup()
}
