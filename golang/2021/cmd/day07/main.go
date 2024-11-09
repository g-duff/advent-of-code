package main

import (
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
	"unicode"
)

func main() {
	file, err := os.ReadFile("./data/day07.input")
	check(err)

	input := parseInput(file)

	ansPt1 := solvePt1(input)
	fmt.Println("Part 1:", ansPt1)

	ansPt2 := solvePt2(input)
	fmt.Println("Part 2:", ansPt2)
}

func solvePt1(input []int) int {
	// Init
	bracketLower := slices.Min(input)
	bracketUpper := slices.Max(input)

	// Start solving
	costs := make([]int, (bracketUpper-bracketLower)+1)

	for i := bracketLower; i <= bracketUpper; i++ {
		costs[i-bracketLower] = costPt1(input, i)
	}

	ans := slices.Min(costs)

	return ans
}

func costPt1(x []int, x0 int) int {
	tot := 0
	for _, xi := range x {
		tot += abs(x0 - xi)
	}
	return tot
}

func abs(x int) int {
	if x >= 0 {
		return x
	} else {
		return (-x)
	}
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
