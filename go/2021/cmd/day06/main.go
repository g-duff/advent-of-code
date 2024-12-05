package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	"unicode"
)

func main() {
	file, err := os.ReadFile("./data/day06.input")
	check(err)

	input := parseInput(file)

	ansPt1 := solve(input, 80)
	fmt.Println("Part 1:", ansPt1)

	ansPt2 := solve(input, 256)
	fmt.Println("Part 2:", ansPt2)
}

func solve(input []int, days int) int {

	var stateCounts [9]int

	for _, n := range input {
		stateCounts[n]++
	}

	for day := 0; day < days; day++ {
		tmp := stateCounts[0]
		//TODO: consider modular arithmetic
		for i := 1; i < 9; i++ {
			stateCounts[i-1] = stateCounts[i]
		}
		stateCounts[8] = tmp
		stateCounts[6] += tmp
	}

	return sum(stateCounts)
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

func sum(l [9]int) int {
	t := 0
	for _, n := range l {
		t += n
	}
	return t
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
