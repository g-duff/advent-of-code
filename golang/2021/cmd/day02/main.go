package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.ReadFile("./data/day02.input")
	check(err)

	input := strings.Split(strings.TrimSpace(string(file)), "\n")

	ansPt1 := solvePt1(input)
	fmt.Println("Part 1: " + strconv.Itoa(ansPt1))

	ansPt2 := solvePt2(input)
	fmt.Println("Part 2: " + strconv.Itoa(ansPt2))
}

func solvePt1(input []string) int {
	h := 0
	v := 0
	for _, line := range input {
		sl := strings.Split(line, " ")
		motion := sl[0]
		count, err := strconv.Atoi(sl[1])
		check(err)

		switch motion {
		case "forward":
			h += count
		case "down":
			v += count
		case "up":
			v -= count
		}
	}

	return h * v
}

func solvePt2(input []string) int {
	h := 0
	v := 0
	a := 0
	for _, line := range input {
		sl := strings.Split(line, " ")
		motion := sl[0]
		count, err := strconv.Atoi(sl[1])
		check(err)

		switch motion {
		case "down":
			a += count
		case "up":
			a -= count
		case "forward":
			h += count
			v += count * a
		}
	}

	return h * v
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
