package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.ReadFile("./data/day10.input")
	check(err)

	topographicMap := parseToIntGrid(file)

	pt1 := solvePt1(topographicMap)
	fmt.Println("Part 1: ", pt1)

	pt2 := solvePt2(topographicMap)
	fmt.Println("Part 2: ", pt2)
}

func solvePt1(topographicMap [][]int) int {
	R := len(topographicMap)
	C := len(topographicMap[0])

	count := 0
	for r, row := range topographicMap {
		for c, cell := range row {
			if cell == 0 {
				ends := []int{}
				ends = append(ends, searchPt1(topographicMap, R, C, 1, r+1, c)...)
				ends = append(ends, searchPt1(topographicMap, R, C, 1, r-1, c)...)
				ends = append(ends, searchPt1(topographicMap, R, C, 1, r, c+1)...)
				ends = append(ends, searchPt1(topographicMap, R, C, 1, r, c-1)...)

				// Get unique endings counts
				endsCount := make(map[int]bool)
				for _, e := range ends {
					endsCount[e] = true
				}

				count += len(endsCount)
			}
		}
	}

	return count
}

// Recursive Depth-First Search returning positions
func searchPt1(topographicMap [][]int, R int, C int, targetHeight int, r int, c int) []int {
	// Bounds check
	if r < 0 || r >= R || c < 0 || c >= C {
		return []int{}
	}

	// Search remaining positions
	height := topographicMap[r][c]
	ends := []int{}
	if height == targetHeight {
		if targetHeight == 9 {
			return []int{R*r + c}
		} else {
			ends = append(ends, searchPt1(topographicMap, R, C, targetHeight+1, r+1, c)...)
			ends = append(ends, searchPt1(topographicMap, R, C, targetHeight+1, r-1, c)...)
			ends = append(ends, searchPt1(topographicMap, R, C, targetHeight+1, r, c+1)...)
			ends = append(ends, searchPt1(topographicMap, R, C, targetHeight+1, r, c-1)...)
		}
	}

	return ends
}

func solvePt2(topographicMap [][]int) int {
	R := len(topographicMap)
	C := len(topographicMap[0])

	count := 0
	for r, row := range topographicMap {
		for c, cell := range row {
			if cell == 0 {
				count += searchPt2(topographicMap, R, C, 1, r+1, c)
				count += searchPt2(topographicMap, R, C, 1, r-1, c)
				count += searchPt2(topographicMap, R, C, 1, r, c+1)
				count += searchPt2(topographicMap, R, C, 1, r, c-1)
			}
		}
	}

	return count
}

// Recursive Depth-First Search counting positions
func searchPt2(topographicMap [][]int, R int, C int, targetHeight int, r int, c int) int {
	// Bounds check
	if r < 0 || r >= R || c < 0 || c >= C {
		return 0
	}

	// Search remaining positions
	height := topographicMap[r][c]
	ends := 0
	if height == targetHeight {
		if targetHeight == 9 {
			return 1
		} else {
			ends += searchPt2(topographicMap, R, C, targetHeight+1, r+1, c)
			ends += searchPt2(topographicMap, R, C, targetHeight+1, r-1, c)
			ends += searchPt2(topographicMap, R, C, targetHeight+1, r, c+1)
			ends += searchPt2(topographicMap, R, C, targetHeight+1, r, c-1)
		}
	}

	return ends
}

func parseToIntGrid(input []byte) [][]int {
	data := strings.TrimSpace(string(input))
	lines := strings.Split(data, "\n")

	grid := make([][]int, len(lines))

	for i, line := range lines {
		grid[i] = make([]int, len(line))
		for j, cell := range line {
			n, err := strconv.Atoi(string(cell))
			check(err)
			grid[i][j] = n
		}
	}
	return grid
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
