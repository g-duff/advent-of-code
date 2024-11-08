package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	"unicode"
)

func main() {
	file, err := os.ReadFile("./data/day07.example")
	check(err)

	input := parseInput(file)

	ansPt1 := solvePt1(input)
	fmt.Println("Part 1:", ansPt1)

	ansPt2 := solvePt2(input)
	fmt.Println("Part 2:", ansPt2)
}

func solvePt1(input []int) int {
	return 0
}

func solvePt2(input []int) int {
	return 0
}

func parseInput(file []byte) []int {
	input := string(file)

	nums := strings.FieldsFunc(input, func(r rune) bool { return !unicode.IsNumber(r) })

	parsed := make([]int, len(nums))
	for i := range nums {
		n, err := strconv.Atoi(nums[i])
		check(err)
		parsed[i] = n
	}
	return parsed
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
