package bootstrap

import (
	"fmt"
	"html/template"
	"os"
	"path"
	"strconv"
)

type TemplateData struct {
	DayNumber int
}

func InitializeDay(day uint64) error {
	fmt.Printf("Initializing day %v\n", day)

	folder, err := createFolder(day)
	if err != nil {
		return err
	}

	err = createMain(folder)
	if err != nil {
		return err
	}

	err = createMainTest(folder)
	if err != nil {
		return err
	}

	return nil
}

func createFolder(day uint64) (string, error) {
	dirName := fmt.Sprintf("day%v", strconv.FormatUint(day, 10))
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

func createMain(folderPath string) error {
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
	fmt.Println(tmpl.Name())

	outFilePath := path.Join(folderPath, "main.go")
	outFile, err := os.Create(outFilePath)
	if err != nil {
		return err
	}
	defer outFile.Close()

	err = tmpl.Execute(outFile, TemplateData{DayNumber: 5})
	if err != nil {
		return err
	}

	return nil
}

func createMainTest(folderPath string) error {
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
	fmt.Println(tmpl.Name())

	outFilePath := path.Join(folderPath, "main_test.go")
	outFile, err := os.Create(outFilePath)
	if err != nil {
		return err
	}
	defer outFile.Close()

	err = tmpl.Execute(outFile, TemplateData{DayNumber: 5})
	if err != nil {
		return err
	}

	return nil
}
