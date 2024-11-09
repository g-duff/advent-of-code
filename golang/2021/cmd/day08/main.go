package main

import (
	"fmt"
	"os"
	"strings"
	"unicode"
)

func main() {
	file, err := os.ReadFile("./data/day08.input")
	check(err)

	input := parseInput(file)

	ansPt1 := solvePt1(input)
	fmt.Println("Part 1:", ansPt1)

	ansPt2 := solvePt2(input)
	fmt.Println("Part 2:", ansPt2)
}

func solvePt1(input [][]string) int {
	ans := 0
	for _, line := range input {
		for i := 10; i < 14; i++ {
			l := len(line[i])
			if l == 2 || l == 4 || l == 3 || l == 7 {
				ans++
			}
		}
	}
	return ans
}

func solvePt2(input [][]string) int {
	return 0
}

func parseInput(file []byte) [][]string {
	input := strings.TrimSpace(string(file))
	inputlines := strings.Split(input, "\n")

	parsed := make([][]string, len(inputlines))

	for i, line := range inputlines {
		fields := strings.FieldsFunc(line, func(r rune) bool { return !unicode.IsLetter(r) })
		parsed[i] = fields
	}
	return parsed
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
