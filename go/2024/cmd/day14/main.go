package main

import (
	"fmt"
	"os"

	"github.com/g-duff/advent-of-code/go/2024/lib"
)

func main() {
	file, err := os.ReadFile("./data/day14.input")
	check(err)

	robotStates := lib.ParseToIntGrid(file)
	check(err)

	pt1 := solvePt1(robotStates)
	fmt.Println("Part 1: ", pt1)

	// Print it out. Search for patterns.
	// Add 100 to the index because pt1 steps 100 iterations.
	solvePt2(robotStates)
}

func solvePt1(robots [][]int) int {

	C := 101
	R := 103

	for i := 0; i < 100; i++ {
		for _, robot := range robots {
			robot[0] += robot[2]
			robot[1] += robot[3]

			// 0 = X = cols
			if robot[0] < 0 {
				robot[0] += C
			}
			if robot[0] >= C {
				robot[0] -= C
			}

			if robot[1] < 1 {
				robot[1] += R
			}
			if robot[1] >= R {
				robot[1] -= R
			}
		}
	}

	counts := [4]int{0, 0, 0, 0}
	for _, robot := range robots {
		if robot[0] < 50 {
			if robot[1] < 51 {
				counts[0] += 1
			}
			if robot[1] > 51 {
				counts[1] += 1
			}
		}
		if robot[0] > 50 {
			if robot[1] < 51 {
				counts[2] += 1
			}
			if robot[1] > 51 {
				counts[3] += 1
			}
		}
	}

	ans := 1
	for _, c := range counts {
		if c != 0 {
			ans *= c
		}
	}

	return ans
}

func solvePt2(robots [][]int) {
	C := 101
	R := 103

	for i := 1; i < 10000; i++ {
		for _, robot := range robots {
			robot[0] += robot[2]
			robot[1] += robot[3]

			// 0 = X = cols
			if robot[0] < 0 {
				robot[0] += C
			}
			if robot[0] >= C {
				robot[0] -= C
			}

			if robot[1] < 1 {
				robot[1] += R
			}
			if robot[1] >= R {
				robot[1] -= R
			}
		}

		fmt.Println("\n\n", i)
		var debugGrid [103][103]int
		for _, robot := range robots {
			debugGrid[robot[1]][robot[0]] = 1
		}
		for _, row := range debugGrid {
			fmt.Println(row)
		}

	}
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
