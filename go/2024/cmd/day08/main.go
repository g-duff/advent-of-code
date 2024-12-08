package main

import (
	"fmt"
	"os"

	"github.com/g-duff/advent-of-code/go/2024/lib"
)

func main() {
	file, err := os.ReadFile("./data/day08.input")
	check(err)

	antennaMap := lib.ParseToRuneGrid(file)

	pt1, pt2 := solve(antennaMap)
	fmt.Println("Part 1: ", pt1)
	fmt.Println("Part 2: ", pt2)
}

type position struct {
	r int
	c int
}

func solve(antennaMap [][]rune) (int, int) {
	allAntennasPositions := make(map[rune][]position)
	for r, row := range antennaMap {
		for c, cell := range row {
			if cell != '.' {
				allAntennasPositions[cell] = append(allAntennasPositions[cell], position{r, c})
			}
		}
	}

	R := len(antennaMap)
	C := len(antennaMap[0])

	antinodePositionsPt1 := make(map[position]bool)
	antinodePositionsPt2 := make(map[position]bool)
	for _, antennaPositions := range allAntennasPositions {
		for i, p1 := range antennaPositions {
			for _, p2 := range antennaPositions[i+1:] {
				var p11, p22 position

				d := position{p2.r - p1.r, p2.c - p1.c}

				/////////
				// Pt1 //
				/////////
				p11 = position{p1.r - d.r, p1.c - d.c}
				if p11.r >= 0 && p11.r < R && p11.c >= 0 && p11.c < C {
					antinodePositionsPt1[p11] = true
				}

				p22 = position{p2.r + d.r, p2.c + d.c}
				if p22.r >= 0 && p22.r < R && p22.c >= 0 && p22.c < C {
					antinodePositionsPt1[p22] = true
				}

				/////////
				// Pt2 //
				/////////
				p11 = position{p1.r, p1.c}
				for p11.r >= 0 && p11.r < R && p11.c >= 0 && p11.c < C {
					antinodePositionsPt2[p11] = true
					p11.r -= d.r
					p11.c -= d.c
				}

				p22 = position{p2.r, p2.c}
				for p22.r >= 0 && p22.r < R && p22.c >= 0 && p22.c < C {
					antinodePositionsPt2[p22] = true
					p22.r += d.r
					p22.c += d.c
				}
			}
		}
	}

	return len(antinodePositionsPt1), len(antinodePositionsPt2)
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
