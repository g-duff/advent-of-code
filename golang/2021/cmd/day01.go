package main

import (
	"fmt"
	"strconv"
	"strings"
	"os"

)

func main() {
	file, err := os.ReadFile("./data/day01.input")
	check(err)

	input := strings.TrimSpace(string(file))

	pt1 := part1(input)

	fmt.Println("Part 1: " + strconv.Itoa(pt1));
	fmt.Println("Part 2: " + "todo");
}

func part1(input string) (int) {

	inputlines := strings.Split(input, "\n")

	L := len(inputlines)
	ints := make([]int, L)

	for i := 0;i<L;i++ {
		si, err := strconv.Atoi(inputlines[i])
		check(err)
		ints[i] = si
	}

	tot := 0
	for i := 0; i<(L-1);i++ {
		if (ints[i+1] > ints[i]) {
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
