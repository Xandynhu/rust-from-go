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

type Output struct {
	Modules Modules `json:"modules"`
}

type Modules struct {
	Declarations map[string]ModuleInfo `json:"declarations"`
	Instances    map[string][]ModuleInfo
	Exports      map[string]ModuleInfo
	Missing      map[string][]ModuleInfo
}

type ModuleInfo struct {
	Name string `json:"name"`
	File string `json:"file"`
	Line int    `json:"line"`
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

	outputJsonString := gate.RunSvParser(inputJsonString)

	// Convert output JSON string to Output struct
	var output Output
	err = json.Unmarshal([]byte(outputJsonString), &output)
	if err != nil {
		fmt.Println("Error unmarshalling output JSON:", err)
		return
	}

}
