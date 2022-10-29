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
)

type TemplateData struct {
	Year      uint64
	DayNumber uint64
}

func InitializeDay(day uint64) error {
	fmt.Printf("Initializing day %v\n", day)

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
	dirName := fmt.Sprintf("y%vd%v", config.Year(), strconv.FormatUint(day, 10))
	fmt.Printf("Creating the day's directory %v\n", dirName)

	curDir, err := os.Getwd()
	if err != nil {
		return "", err
	}

	dayDir := path.Join(curDir, dirName)
	err = os.Mkdir(dayDir, os.ModePerm)
	if err != nil {
		return "", err
	}

	return dayDir, nil
}

func createMain(day uint64, folderPath string) error {
	fmt.Println("Creating the day's main file")

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

	outFilePath := path.Join(folderPath, "main.go")
	outFile, err := os.Create(outFilePath)
	if err != nil {
		return err
	}
	defer outFile.Close()

	err = tmpl.Execute(outFile, TemplateData{
		DayNumber: day,
		Year:      config.Year(),
	})
	if err != nil {
		return err
	}

	return nil
}

func createMainTest(day uint64, folderPath string) error {
	fmt.Println("Creating the day's main_test file")

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

	outFilePath := path.Join(folderPath, "main_test.go")
	outFile, err := os.Create(outFilePath)
	if err != nil {
		return err
	}
	defer outFile.Close()

	err = tmpl.Execute(outFile, TemplateData{
		DayNumber: day,
		Year:      config.Year(),
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
