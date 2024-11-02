package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.ReadFile("./data/day01.input")
	check(err)

	input := strings.TrimSpace(string(file))

	pt1 := part1(input)

	fmt.Println("Part 1: " + strconv.Itoa(pt1))

	pt2 := part2(input)
	fmt.Println("Part 2: " + strconv.Itoa(pt2))
}

func part1(input string) int {

	inputlines := strings.Split(input, "\n")

	L := len(inputlines)
	ints := make([]int, L)

	for i := 0; i < L; i++ {
		si, err := strconv.Atoi(inputlines[i])
		check(err)
		ints[i] = si
	}

	tot := 0
	for i := 0; i < (L - 1); i++ {
		if ints[i+1] > ints[i] {
			tot += 1
		}
	}

	return tot
}

func part2(input string) int {
	inputlines := strings.Split(input, "\n")

	L := len(inputlines)
	ints := make([]int, L)
	for i := 0; i < L; i++ {
		si, err := strconv.Atoi(inputlines[i])
		check(err)
		ints[i] = si
	}

	L -= 2
	avgs := make([]int, L)
	for i := 0; i < L; i++ {
		avgs[i] = ints[i] + ints[i+1] + ints[i+2]
	}

	tot := 0
	L -= 1
	for i := 0; i < L; i++ {
		if avgs[i+1] > avgs[i] {
			tot += 1
		}
	}

	return tot
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
