package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	file, err := os.ReadFile("./data/day15.input")
	check(err)

	warehouseMap, movements := parse(file)

	pt1 := solvePt1(warehouseMap, movements)
	fmt.Println("Part 1:", pt1)
}

func solvePt1(warehouseMap [][]rune, movements []rune) int {

	var botR, botC int
	for r, row := range warehouseMap {
		for c, cell := range row {
			if cell == '@' {
				botR, botC = r, c
			}
		}
	}

	var dr, dc int
	for _, m := range movements {
		switch m {
		case '^':
			dr, dc = -1, 0
		case 'v':
			dr, dc = 1, 0
		case '<':
			dr, dc = 0, -1
		case '>':
			dr, dc = 0, 1
		}
		if warehouseMap[botR+dr][botC+dc] == '.' {
			warehouseMap[botR][botC] = '.'
			botR += dr
			botC += dc
			warehouseMap[botR][botC] = '@'
		} else if warehouseMap[botR+dr][botC+dc] == 'O' {
			sf := 1
			for true {
				if warehouseMap[botR+sf*dr][botC+sf*dc] == '.' {
					// Move the boxes
					warehouseMap[botR+dr][botC+dc] = '.'
					warehouseMap[botR+sf*dr][botC+sf*dc] = 'O'

					// Move the robot
					warehouseMap[botR][botC] = '.'
					botR += dr
					botC += dc
					warehouseMap[botR][botC] = '@'

					break
				}

				if warehouseMap[botR+sf*dr][botC+sf*dc] == '#' {
					break
				}
				sf++
			}
		}
	}

	ans := 0
	for r, row := range warehouseMap {
		for c, cell := range row {
			if cell == 'O' {
				ans += (r*100 + c)
			}
		}
	}
	return ans
}

func parse(input []byte) ([][]rune, []rune) {
	line := strings.Split(strings.TrimSpace(string(input)), "\n\n")

	rawMap := line[0]
	rawInstructions := []rune(strings.Replace(line[1], "\n", "", -1))

	mapLines := strings.Split(rawMap, "\n")
	grid := make([][]rune, len(mapLines))
	for i, line := range mapLines {
		grid[i] = []rune(line)
	}

	return grid, rawInstructions
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
