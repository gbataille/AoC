package cmd

import (
	"strconv"

	"github.com/pkg/errors"
	"github.com/spf13/cobra"

	"github.com/gbataille/AoC_2022/internal/cmd/clean"
)

// bootstrapCmd represents the bootstrap command
var cleanCmd = &cobra.Command{
	Use:          "clean",
	Short:        "Cleans a problem's directory",
	SilenceUsage: true,
	RunE: func(cmd *cobra.Command, _ []string) (err error) {
		var day uint64
		dayStr := cmd.Flag(dayFlag).Value.String()
		day, err = strconv.ParseUint(dayStr, 10, 64)
		if err != nil {
			return errors.Errorf("invalid argument '%v' %v. Must be a number between 1 and 25\n", dayFlag, dayStr)
		}

		err = clean.CleanDay(day)
		return err
	},
}

func init() {
	rootCmd.AddCommand(cleanCmd)

	cleanCmd.Flags().IntP(dayFlag, "d", 0, "The day to clean")
	cleanCmd.MarkFlagRequired(dayFlag)
}
