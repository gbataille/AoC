package clean

import (
	"os"

	"github.com/gbataille/AoC_2022/internal/logging"
	"github.com/gbataille/AoC_2022/internal/problems"
)

func CleanDay(day uint64) error {
	logging.Logger.Infof("Cleaning day %v", day)

	removeFolder(day)

	return nil
}

func removeFolder(day uint64) error {
	dayDirName, err := problems.DirectoryForDay(day)
	if err != nil {
		return err
	}

	logging.Logger.Infof("Removing the day's directory %v", dayDirName)

	err = os.RemoveAll(dayDirName)
	if err != nil {
		return err
	}

	return nil
}
