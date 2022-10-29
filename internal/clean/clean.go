package clean

import (
	"fmt"
	"os"
	"path"
	"strconv"
)

func CleanDay(day uint64) error {
	fmt.Printf("Initializing day %v\n", day)

	removeFolder(day)

	return nil
}

func removeFolder(day uint64) error {
	dirName := fmt.Sprintf("day%v", strconv.FormatUint(day, 10))
	fmt.Printf("Removing the day's directory %v\n", dirName)

	curDir, err := os.Getwd()
	if err != nil {
		return err
	}

	dayDir := path.Join(curDir, dirName)
	err = os.RemoveAll(dayDir)
	if err != nil {
		return err
	}

	return nil
}
