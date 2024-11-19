package main

import (
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

func main() {
	file, err := os.ReadFile("./data/day09.input")
	check(err)

	input := parseInput(file)

	ansPt1 := solvePt1(input)
	fmt.Println("Part 1:", ansPt1)

	ansPt2 := solvePt2(input)
	fmt.Println("Part 2:", ansPt2)
}

func solvePt1(h HeightMap) int {
	riskLevel := 0
	for _, lowPoint := range h.findLowPoints() {
		r, c := lowPoint[0], lowPoint[1]
		riskLevel += (1 + h.getOr(r, c, 0))
	}
	return riskLevel
}

func solvePt2(input HeightMap) int {

	lowPoints := input.findLowPoints()

	sizes := make([]int, len(lowPoints))

	for i, lowPoint := range lowPoints {
		state := make([][]int, input.rows)
		for i :=0; i<input.rows; i++ {
			state[i] = make([]int, input.cols)
		}

		r, c := lowPoint[0], lowPoint[1]
		state[r][c] = 1
		floodFill(input, state, [2]int{r+1, c}, input.grid[r][c])
		floodFill(input, state, [2]int{r-1, c}, input.grid[r][c])
		floodFill(input, state, [2]int{r, c+1}, input.grid[r][c])
		floodFill(input, state, [2]int{r, c-1}, input.grid[r][c])

		tot := 0
		for _, row := range state {
			for _, cell := range row {
				tot += cell
			}
		}
		sizes[i] = tot
	}

	sort.Ints(sizes)
	L := len(sizes)
	return sizes[L-1]*sizes[L-2]*sizes[L-3]
}

func floodFill(h HeightMap, state [][]int, loc [2]int, val int) {
	r, c := loc[0], loc[1]

	if r < 0 || r >= h.rows  || c < 0 || c >= h.cols {
		return
	}

	currentPos := h.grid[r][c]
	isSet := state[r][c] == 1
	if isSet || currentPos < val || currentPos == 9 {
		return
	}

	state[r][c] = 1

	floodFill(h, state, [2]int{r+1, c}, currentPos)
	floodFill(h, state, [2]int{r-1, c}, currentPos)
	floodFill(h, state, [2]int{r, c+1}, currentPos)
	floodFill(h, state, [2]int{r, c-1}, currentPos)
}

type HeightMap struct {
	grid [][]int
	rows int
	cols int
}

func (h *HeightMap) findLowPoints() [][2]int {
	var rows = [][2]int{}
	for r := 0; r < h.rows; r++ {
		for c := 0; c < h.cols; c++ {
			if h.isLowPoint(r, c) {
				rows = append(rows, [2]int{r, c})
			}
		}
	}
	return rows
}

func (h *HeightMap) isLowPoint(row int, col int) bool {
	mid := h.grid[row][col]

	return mid < h.getOr(row-1, col, 10) &&
		mid < h.getOr(row+1, col, 10) &&
		mid < h.getOr(row, col-1, 10) &&
		mid < h.getOr(row, col+1, 10)
}

func (h *HeightMap) getOr(row int, col int, default_value int) int {
	if row < 0 || row >= h.rows {
		return default_value
	}

	if col < 0 || col >= h.cols {
		return default_value
	}

	return h.grid[row][col]
}

func parseInput(file []byte) HeightMap {
	input := string(file)

	inputlines := strings.Split(strings.TrimSpace(input), "\n")

	rows := len(inputlines)
	cols := len(inputlines[0])

	grid := make([][]int, rows)

	for row, line := range inputlines {
		grid[row] = make([]int, cols)
		for col, char := range line {
			n, err := strconv.Atoi(string(char))
			check(err)
			grid[row][col] = n
		}
	}
	return HeightMap{grid, rows, cols}
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
