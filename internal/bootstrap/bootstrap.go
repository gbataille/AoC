package bootstrap

import (
	"fmt"
	"html/template"
	"io"
	"net/http"
	"os"
	"path"
	"strconv"

	"github.com/gbataille/AoC_2022/internal/config"
	"github.com/gbataille/AoC_2022/internal/logging"
	"github.com/gbataille/AoC_2022/internal/problems"
)

type TemplateData struct {
	DayNumber uint64
}

func InitializeDay(day uint64) error {
	logging.Logger.Infof("Initializing day %v", day)

	folder, err := createFolder(day)
	if err != nil {
		return err
	}

	err = createMain(day, folder)
	if err != nil {
		return err
	}

	err = createMainTest(day, folder)
	if err != nil {
		return err
	}

	err = retrieveInput(day, folder)
	if err != nil {
		return err
	}

	return nil
}

func createFolder(day uint64) (string, error) {
	dayDirPath, err := problems.DirectoryForDay(day)
	if err != nil {
		return "", err
	}

	yearDirPath := path.Dir(dayDirPath)
	err = os.Mkdir(yearDirPath, os.ModeDir|os.ModePerm)
	if err != nil {
		if os.IsExist(err) {
			logging.Logger.Infof("Year folder %s already exists", yearDirPath)
		} else {
			return "", err
		}
	} else {
		logging.Logger.Infof("Created year folder %s", yearDirPath)
	}

	dayDirName := fmt.Sprintf("d%v", strconv.FormatUint(day, 10))
	logging.Logger.Infof("Creating the day's directory %v", dayDirName)

	err = os.Mkdir(dayDirPath, os.ModePerm)
	if err != nil {
		return "", err
	}

	return dayDirPath, nil
}

func createMain(day uint64, folderPath string) error {
	logging.Logger.Infof("Creating the day's main file")

	const TemplateName = "day.main.go.tmpl"

	workDir, err := os.Getwd()
	if err != nil {
		return err
	}
	templatePath := path.Join(workDir, "assets", TemplateName)

	tmpl, err := template.ParseFiles(templatePath)
	if err != nil {
		return err
	}

	outFilePath := path.Join(folderPath, fmt.Sprintf("d%d.go", day))
	outFile, err := os.Create(outFilePath)
	if err != nil {
		return err
	}
	defer outFile.Close()

	err = tmpl.Execute(outFile, TemplateData{
		DayNumber: day,
	})
	if err != nil {
		return err
	}

	return nil
}

func createMainTest(day uint64, folderPath string) error {
	logging.Logger.Infoln("Creating the day's main_test file")

	const TemplateName = "day.main_test.go.tmpl"

	workDir, err := os.Getwd()
	if err != nil {
		return err
	}
	templatePath := path.Join(workDir, "assets", TemplateName)

	tmpl, err := template.ParseFiles(templatePath)
	if err != nil {
		return err
	}

	outFilePath := path.Join(folderPath, fmt.Sprintf("d%d_test.go", day))
	outFile, err := os.Create(outFilePath)
	if err != nil {
		return err
	}
	defer outFile.Close()

	err = tmpl.Execute(outFile, TemplateData{
		DayNumber: day,
	})
	if err != nil {
		return err
	}

	return nil
}

func retrieveInput(day uint64, folderPath string) error {
	const fileName = "input.txt"
	filePath := path.Join(folderPath, fileName)

	file, err := os.Create(filePath)
	if err != nil {
		return err
	}
	defer file.Close()

	client := http.Client{}
	baseUrl := config.AocUrl()
	if baseUrl[len(baseUrl)-1] == '/' {
		baseUrl = baseUrl[:len(baseUrl)-1]
	}
	fullURLFile := fmt.Sprintf("%v/%v/day/%v/input", baseUrl, config.Year(), day)

	req, err := http.NewRequest(http.MethodGet, fullURLFile, nil)
	if err != nil {
		return err
	}
	req.AddCookie(&http.Cookie{
		Name:     "session",
		Value:    config.AocSessionKey(),
		Path:     "/",
		Domain:   ".adventofcode.com",
		Secure:   true,
		HttpOnly: true,
	})

	resp, err := client.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()

	_, err = io.Copy(file, resp.Body)
	if err != nil {
		return err
	}

	return nil
}
