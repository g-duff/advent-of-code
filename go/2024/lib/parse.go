package lib

import (
	"strconv"
	"strings"
	"unicode"
)

func ParseToIntGrid(input []byte) [][]int {
	data := strings.TrimSpace(string(input))
	lines := strings.Split(data, "\n")

	grid := make([][]int, len(lines))

	for i, line := range lines {
		fields := strings.FieldsFunc(line, func(r rune) bool { return (!unicode.IsNumber(r) && r != '-' ) })
		grid[i] = make([]int, len(fields))
		for j, cell := range fields {
			n, err := strconv.Atoi(cell)
			check(err)
			grid[i][j] = n
		}
	}
	return grid
}

func ParseToRuneGrid(input []byte) [][]rune {
	data := strings.TrimSpace(string(input))
	lines := strings.Split(data, "\n")

	grid := make([][]rune, len(lines))
	for i, line := range lines {
		grid[i] = []rune(line)
	}

	return grid
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
