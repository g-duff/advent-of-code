package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	file, err := os.ReadFile("./data/day05.example")
	input := strings.Split(strings.TrimSpace(string(file)), "\n")
	check(err)

	ansPt1 := solvePt1(input)
	fmt.Println("Part 1:", ansPt1)

	ansPt2 := solvePt2(input)
	fmt.Println("Part 2:", ansPt2)
}

func solvePt1(input []string) int {
	return 0
}

func solvePt2(input []string) int {
	return 0
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
