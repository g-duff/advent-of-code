package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	"unicode"
)

func main() {
	file, err := os.ReadFile("./data/day11.input")
	check(err)

	stones := parse(file)

	pt1 := solve(stones, 25)
	fmt.Println("Part 1: ", pt1)

	pt2 := solve(stones, 75)
	fmt.Println("Part 2: ", pt2)
}

func solve(stones []int, blinks int) int {
	stonesCounts := make(map[int]int)

	for _, s := range stones {
		stonesCounts[s] += 1
	}

	for b := 0; b < blinks; b++ {
		nextStonesCounts := make(map[int]int)

		for stone, count := range stonesCounts {
			if stone == 0 {
				nextStonesCounts[1] += count
			} else if len(strconv.Itoa(stone))%2 == 0 {
				s := strconv.Itoa(stone)
				l := len(s) / 2
				s1, err1 := strconv.Atoi(s[:l])
				s2, err2 := strconv.Atoi(s[l:])

				check(err1)
				check(err2)

				nextStonesCounts[s1] += count
				nextStonesCounts[s2] += count
			} else {
				nextStonesCounts[stone*2024] += count
			}
		}
		stonesCounts = nextStonesCounts
	}

	total := 0
	for _, count := range stonesCounts {
		total += count
	}

	return total
}

func parse(input []byte) []int {
	line := strings.TrimSpace(string(input))

	fields := strings.FieldsFunc(line, func(r rune) bool { return !unicode.IsNumber(r) })
	nums := make([]int, len(fields))

	for i, f := range fields {
		n, err := strconv.Atoi(f)
		check(err)
		nums[i] = n
	}

	return nums
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
