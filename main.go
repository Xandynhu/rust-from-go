package main

import (
	"encoding/json"
	"fmt"

	gate "rust-from-go/src"
)

type Input struct {
	Files Files `json:"files"`
}

type Files struct {
	Include []string `json:"include"`
	Source  []string `json:"source"`
}

func main() {
	// Example input
	input := Input{
		Files: Files{
			Include: []string{},
			Source: []string{
				"test/00-all-ports/PortModules.sv",
				"test/00-all-ports/TopModule.sv",
			},
		},
	}

	// Convert input to JSON string
	inputJson, err := json.Marshal(input)
	if err != nil {
		fmt.Println("Error marshalling input to JSON:", err)
		return
	}

	inputJsonString := string(inputJson)
	// fmt.Println(inputJsonString)

	gate.RunSvParser(inputJsonString)
	// fmt.Println(res)
}
