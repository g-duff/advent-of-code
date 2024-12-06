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
	file, err := os.ReadFile("./data/day01.input")
	check(err)

	col1, col2 := parse(file)

	slices.Sort(col1)
	slices.Sort(col2)

	pt1 := solvePt1(col1, col2)
	fmt.Println("Part 1: ", pt1)

	pt2 := solvePt2(col1, col2)
	fmt.Println("Part 2: ", pt2)
}

func solvePt1(col1 []int, col2 []int) int {
	totalDistance := 0
	for i := range col1 {
		d := col2[i] - col1[i]
		if d >= 0 {
			totalDistance += d
		} else {
			totalDistance -= d
		}
	}
	return totalDistance
}

func solvePt2(col1 []int, col2 []int) int {
	similarityScore := 0

	counts := make(map[int]int)
	for _, n := range col2 {
		counts[n]++
	}

	for _, c := range col1 {
		similarityScore += c * counts[c]
	}
	return similarityScore
}

func parse(input []byte) ([]int, []int) {
	data := strings.TrimSpace(string(input))
	lines := strings.Split(data, "\n")

	col1 := make([]int, len(lines))
	col2 := make([]int, len(lines))

	for i, line := range lines {
		fields := strings.FieldsFunc(line, func(r rune) bool { return !unicode.IsNumber(r) })
		n1, err1 := strconv.Atoi(fields[0])
		check(err1)
		col1[i] = n1

		n2, err2 := strconv.Atoi(fields[1])
		check(err2)
		col2[i] = n2
	}

	return col1, col2
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
