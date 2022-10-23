package bootstrap

import (
	"fmt"
	"os"
	"path"
	"strconv"
)

func InitializeDay(day uint64) error {
	fmt.Printf("Initializing day %v\n", day)

	createFolder(day)

	return nil
}

func createFolder(day uint64) error {
	dirName := fmt.Sprintf("day%v", strconv.FormatUint(day, 10))
	fmt.Printf("Creating the day's directory %v\n", dirName)

	curDir, err := os.Getwd()
	if err != nil {
		return err
	}

	dayDir := path.Join(curDir, dirName)
	err = os.Mkdir(dayDir, os.ModePerm)
	if err != nil {
		return err
	}

	return nil
}