package main

import (
	"fmt"
	"os"

	"github.com/g-duff/advent-of-code/go/2024/lib"
)

func main() {
	file, err := os.ReadFile("./data/day06.input")
	check(err)

	guardMap := lib.ParseToRuneGrid(file)

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
	pc.add(r, c)
	for newR >= 0 && newR < R && newC >= 0 && newC < C {
		if guardMap[newR][newC] != '#' {
			r, c = newR, newC
			newR += dirs[dirIdx][0]
			newC += dirs[dirIdx][1]
			pc.add(r, c)
		} else {
			dirIdx = (dirIdx + 1) % 4
			newR = r + dirs[dirIdx][0]
			newC = c + dirs[dirIdx][1]
		}
	}

	return len(pc)
}

func solvePt2(guardMap [][]rune) int {
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
	pc.add(r, c)
	for newR >= 0 && newR < R && newC >= 0 && newC < C {
		if guardMap[newR][newC] != '#' {
			r, c = newR, newC
			newR += dirs[dirIdx][0]
			newC += dirs[dirIdx][1]
			pc.add(r, c)
		} else {
			dirIdx = (dirIdx + 1) % 4
			newR = r + dirs[dirIdx][0]
			newC = c + dirs[dirIdx][1]
		}
	}

	infiniteStates := 0

	for _, p := range pc[1:] {
		guardMap[p[0]][p[1]] = '#'
		infiniteStates += isInfinite(guardMap)
		guardMap[p[0]][p[1]] = '.'
	}

	return infiniteStates
}

func isInfinite(guardMap [][]rune) int {
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
	states := [][4]int{}
	for true {
		if guardMap[newR][newC] != '#' {
			r, c = newR, newC
			newR += dirs[dirIdx][0]
			newC += dirs[dirIdx][1]
		} else {
			dirIdx = (dirIdx + 1) % 4
			newR = r + dirs[dirIdx][0]
			newC = c + dirs[dirIdx][1]
		}

		if newR < 0 || newR >= R || newC < 0 || newC >= C {
			return 0
		}

		if statesContains(states, [4]int{newR, newC, dirs[dirIdx][0], dirs[dirIdx][1]}) {
			return 1
		} else {
			states = append(states, [4]int{newR, newC, dirs[dirIdx][0], dirs[dirIdx][1]})
		}
	}

	return 0
}

func statesContains(states [][4]int, state [4]int) bool {
	for _, s := range states {
		if s[0] == state[0] && s[1] == state[1] && s[2] == state[2] && s[3] == state[3] {
			return true
		}
	}
	return false
}

type uniquePositions [][2]int

func (u *uniquePositions) add(r int, c int) {
	for _, upos := range *u {
		if upos[0] == r && upos[1] == c {
			return
		}
	}
	*u = append(*u, [2]int{r, c})
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
