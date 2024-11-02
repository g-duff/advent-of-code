package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.ReadFile("./data/day03.input")
	check(err)

	input := strings.Split(strings.TrimSpace(string(file)), "\n")

	pt1 := solvePt1(input)

	fmt.Println("Part 1: " + strconv.Itoa(pt1))
	fmt.Println("Part 2: " + "todo")
}

func solvePt1(input []string) int {
	R := len(input)
	C := len(input[0])

	counts := make([]int, C)

	for _, row := range input {
		for j, c := range row {
			x, err := strconv.Atoi(string(c))
			check(err)
			counts[j] += x
		}
	}

	gamma := 0
	epsilon := 0
	for i, c := range counts {
		if c*2 > R {
			gamma += 1 << (C-i-1)
		} else {
			epsilon += 1 << (C-i-1)
		}
	}

	return gamma * epsilon
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
