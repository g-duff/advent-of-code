package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	"unicode"
)

func main() {
	file, err := os.ReadFile("./data/day05.input")
	check(err)

	input := parseInput(file)

	ansPt1 := solvePt1(input)
	fmt.Println("Part 1:", ansPt1)

	ansPt2 := solvePt2(input)
	fmt.Println("Part 2:", ansPt2)
}

func solvePt1(input []LineSegment) int {
	var grid [1000][1000]int16

	for _, l := range input {

		dx := direction(l.x2 - l.x1)
		dy := direction(l.y2 - l.y1)

		if dy == 0 {
			y := l.y1
			for x := l.x1; x != l.x2+dx; x += dx {
				grid[y][x] += 1
			}
		}

		if dx == 0 {
			x := l.x1
			for y := l.y1; y != l.y2+dy; y += dy {
				grid[y][x] += 1
			}
		}

	}

	tot := 0
	for _, row := range grid {
		for _, cell := range row {
			if cell > 1 {
				tot++
			}
		}
	}
	return tot
}

func solvePt2(input []LineSegment) int {
	var grid [1000][1000]int16

	for _, l := range input {

		dx := direction(l.x2 - l.x1)
		dy := direction(l.y2 - l.y1)

		if dy == 0 {
			y := l.y1
			for x := l.x1; x != l.x2+dx; x += dx {
				grid[y][x] += 1
			}
		} else if dx == 0 {
			x := l.x1
			for y := l.y1; y != l.y2+dy; y += dy {
				grid[y][x] += 1
			}
		} else {
			x := l.x1
			for y := l.y1; y != l.y2+dy; y += dy {
				grid[y][x] += 1
				x += dx
			}
		}

	}

	tot := 0
	for _, row := range grid {
		for _, cell := range row {
			if cell > 1 {
				tot++
			}
		}
	}
	return tot
}

type LineSegment struct {
	x1 int16
	y1 int16
	x2 int16
	y2 int16
}

func direction(n int16) int16 {
	if n == 0 {
		return 0
	} else if uint16(n)&0x8000 == 0x8000 {
		return -1
	} else {
		return 1
	}
}

func parseInput(file []byte) []LineSegment {
	input := strings.Split(strings.TrimSpace(string(file)), "\n")
	parsedInput := make([]LineSegment, len(input))

	for l, line := range input {
		fields := strings.FieldsFunc(line, func(r rune) bool { return !unicode.IsNumber(r) })
		x1, err0 := strconv.ParseInt(fields[0], 10, 16)
		y1, err1 := strconv.ParseInt(fields[1], 10, 16)
		x2, err2 := strconv.ParseInt(fields[2], 10, 16)
		y2, err3 := strconv.ParseInt(fields[3], 10, 16)

		check(err0)
		check(err1)
		check(err2)
		check(err3)

		parsedInput[l] = LineSegment{int16(x1), int16(y1), int16(x2), int16(y2)}
	}
	return parsedInput
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
