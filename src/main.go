package main

import (
	"fmt"
	"os"
	"path/filepath"
)

func main() {
	var filePath string
	if len(os.Args) >= 2{
		filePath = os.Args[1]
	}
	filePathNames := getFileName(filePath)
	writeOutputFile(filePathNames)
}

func getFileName(filePath string) []string {
	if filePath == ""{
		filePath, _ = os.Getwd()
	}
	
	fmt.Printf("Target path: %s\n", filePath)

	filePathNames, err := filepath.Glob(filepath.Join(filePath, "*"))
	if err != nil {
		fmt.Printf("Error: Failed to find the directory: %s", err)
	}
	
	return filePathNames
}

func writeOutputFile(filePathNames []string){
	outputFileName := "output.txt"
	outputFile, err := os.Create(outputFileName)
	if err != nil{
		fmt.Println("Error: Failed to create file. ", err)
	}

	for _, filePath := range filePathNames{
		// filepath.Base() 返回路徑的最後一個 element，即檔案名稱
		outputFile.WriteString(filepath.Base(filePath) + "\n")
	}

	pwd, _ := os.Getwd()
	fmt.Printf("Output file generated in the %s\n", filepath.Join(pwd, outputFileName))
    defer outputFile.Close()
}
