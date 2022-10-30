package problems

import (
	"fmt"
	"os"
	"path"
	"strconv"

	"github.com/gbataille/AoC_2022/internal/config"
)

func DirectoryForDay(day uint64) (string, error) {
	curDir, err := os.Getwd()
	if err != nil {
		return "", err
	}

	yearDirPath := path.Join(curDir, "problems", fmt.Sprintf("y%v", config.Year()), fmt.Sprintf("d%v", strconv.FormatUint(day, 10)))
	return yearDirPath, nil
}
