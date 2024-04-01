package main

import (
	"fmt"
	gate "rust-from-go/src"
)

func Run() {
	res := gate.ProcessJSON("aaa")
	fmt.Println(res)
}

func main() {
	Run()
}
