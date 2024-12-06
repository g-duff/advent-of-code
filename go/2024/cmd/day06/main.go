package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	file, err := os.ReadFile("./data/day06.input")
	check(err)

	guardMap := parse(file)

	pt1 := solvePt1(guardMap)
	fmt.Println("Part 1: ", pt1)

	pt2 := solvePt2(guardMap)
	fmt.Println("Part 2: ", pt2)
}

func solvePt1(guardMap [][]rune) int {
	dirs := [][]int{
		{-1, 0},
		{0, 1},
		{1, 0},
		{0, -1},
	}

	dirIdx := 0
	var r, c, newR, newC int

	for rIdx, row := range guardMap {
		for cIdx, cell := range row {
			if cell == '^' {
				r = rIdx
				c = cIdx
			}
		}
	}

	newR = r + dirs[dirIdx][0]
	newC = c + dirs[dirIdx][1]

	R, C := len(guardMap), len(guardMap[0])

	pc := uniquePositions{}
	pc.add(position{r, c})
	for newR >= 0 && newR < R && newC >= 0 && newC < C {
		if guardMap[newR][newC] != '#' {
			r, c = newR, newC
			newR += dirs[dirIdx][0]
			newC += dirs[dirIdx][1]
			pc.add(position{r, c})
		} else {
			dirIdx = (dirIdx + 1) % 4
			newR = r + dirs[dirIdx][0]
			newC = c + dirs[dirIdx][1]
		}
	}

	return len(pc)
}

func solvePt2(guardMap [][]rune) int {
	ans := 0
	return ans
}

type position struct {
	r int
	c int
}

type uniquePositions []position

func (u *uniquePositions) add(pos position) {
	for _, upos := range *u {
		if upos.r == pos.r && upos.c == pos.c {
			return
		}
	}
	*u = append(*u, pos)
}

func parse(input []byte) [][]rune {
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
