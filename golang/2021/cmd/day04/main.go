package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.ReadFile("./data/day04.example")
	check(err)

	ansPt1 := solvePt1(file)
	fmt.Println("Part 1:", ansPt1)

	ansPt2 := solvePt2(file)
	fmt.Println("Part 2:", ansPt2)
}

func solvePt1(input []byte) int {
	calledNumbers, boards := parseInput(input)

	hasWon := false
	finalNumCalled := 0
	total := 0
	for _, c := range calledNumbers {
		for bIdx := range boards {
			boards[bIdx].playNumber(c)

			if boards[bIdx].hasWon() {
				total = boards[bIdx].countScore()
				hasWon = true
				finalNumCalled = c
				break
			}
		}
		if hasWon {
			break
		}

	}

	return total * finalNumCalled
}

func solvePt2(input []byte) int {
	calledNumbers, boards := parseInput(input)

	totalBoards := len(boards)
	wonBoards := 0

	ans := 0

	for _, c := range calledNumbers {
		for bIdx := range boards {
			if boards[bIdx].IsDone {
				continue
			}

			boards[bIdx].playNumber(c)
			if boards[bIdx].hasWon() {
				wonBoards += 1

				if (wonBoards == totalBoards) {
					ans = boards[bIdx].countScore() * c
				}
			}
		}
	}

	return ans
}

type Board struct {
	nums [5][5]int
	markedPositions [5][5]bool
	IsDone bool
}

func (b *Board) playNumber(n int) {
	for rIdx, row := range b.nums {
		for cIdx, cell := range row {
			if cell == n {
				b.markedPositions[rIdx][cIdx] = true
			}
		}
	}
}

func (b *Board) hasWon() bool {
	var rowsHaveWon, colsHaveWon [5]bool

	for i := 0; i<5 ; i++ {
		rowsHaveWon[i] = true
		colsHaveWon[i] = true
	}

	for rIdx := 0; rIdx < 5; rIdx++ {
		for cIdx := 0; cIdx < 5; cIdx++ {
			rowsHaveWon[rIdx] = rowsHaveWon[rIdx] && b.markedPositions[rIdx][cIdx]
			colsHaveWon[cIdx] = colsHaveWon[cIdx] && b.markedPositions[rIdx][cIdx]
		}
	}

	rowHasWon, colHasWon := false, false
	for i := 0; i<5 ; i++ {
		rowHasWon = rowHasWon || rowsHaveWon[i]
		colHasWon = colHasWon || colsHaveWon[i]
	}

	b.IsDone = rowHasWon || colHasWon
	
	return rowHasWon || colHasWon
}

func (b *Board) countScore() int {
	total := 0
	for rIdx:=0;rIdx<5;rIdx++ {
		for cIdx:=0;cIdx<5;cIdx++ {
			if !b.markedPositions[rIdx][cIdx] {
				total += b.nums[rIdx][cIdx]
			}
		}
	}
	return total
}

func parseInput(input []byte) ([]int, []Board) {
	rows := strings.Split(strings.TrimSpace(string(input)), "\n\n")

	called_numbers_strings := strings.Split(rows[0], ",")
	called_numbers := make([]int, len(called_numbers_strings))
	for i, s := range called_numbers_strings {
		n, err := strconv.Atoi(strings.TrimSpace(s))
		check(err)
		called_numbers[i] = n
	}

	grids_strings := rows[1:]
	grids := make([]Board, len(rows[1:]))

	for i, g := range grids_strings {

		var nums [5][5]int
		var markedPositions [5][5]bool
		for r, gRow := range strings.Split(g, "\n") {
			for c, cCell := range strings.Fields(gRow) {
				n, err := strconv.Atoi(cCell)
				check(err)
				nums[r][c] = n
				markedPositions[r][c] = false
			}
		}
		
		grids[i] = Board{ nums, markedPositions, false}
	}

	return called_numbers, grids
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
