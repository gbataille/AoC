package cmd

import (
	"strconv"

	"github.com/pkg/errors"
	"github.com/spf13/cobra"

	"github.com/gbataille/AoC_2022/internal/bootstrap"
)

// bootstrapCmd represents the bootstrap command
var bootstrapCmd = &cobra.Command{
	Use:   "bootstrap",
	Short: "Creates the skeleton for a problem",
	Long: `For each problem, creates
- a templated main file to be run
- a templated test file to be run (for unit test or as an alternative quick code launcher)
- the downloaded input file for the problem
`,
	SilenceUsage: true,
	RunE: func(cmd *cobra.Command, args []string) (err error) {
		var day uint64
		dayStr := cmd.Flag(dayFlag).Value.String()
		day, err = strconv.ParseUint(dayStr, 10, 64)
		if err != nil {
			return errors.Errorf("invalid argument '%v' %v. Must be a number between 1 and 25\n", dayFlag, dayStr)
		}

		err = bootstrap.InitializeDay(day)
		return err
	},
}

func init() {
	rootCmd.AddCommand(bootstrapCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// bootstrapCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	bootstrapCmd.Flags().IntP(dayFlag, "d", 0, "The day for which to generate the problem template")
	bootstrapCmd.MarkFlagRequired(dayFlag)
}
