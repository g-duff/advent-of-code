package main

import (
	"fmt"
	"os"

	"github.com/g-duff/advent-of-code/go/2024/lib"
)

func main() {
	file, err := os.ReadFile("./data/day02.input")
	check(err)

	reports := lib.ParseToIntGrid(file)

	pt1 := solvePt1(reports)
	fmt.Println("Part 1: ", pt1)

	pt2 := solvePt2(reports)
	fmt.Println("Part 2: ", pt2)
}

func solvePt1(reports [][]int) int {
	safeReports := 0
	for _, report := range reports {
		if isSafe(report) {
			safeReports++
		}
	}
	return safeReports
}

func solvePt2(reports [][]int) int {
	safeReports := 0
	for _, report := range reports {
		if isSafe(report) {
			safeReports++
		} else {
			other := make([]int, len(report))
			for i := 0; i < len(report); i++ {
				// I still don't understand slices
				copy(other, report)
				if isSafe(append(other[:i], other[i+1:]...)) {
					safeReports++
					break
				}
			}
		}
	}
	return safeReports
}

func isSafe(report []int) bool {
	isAllPositive := true
	isAllNegative := true
	isCorrectSize := true

	N := len(report) - 1
	for i := 0; i < N; i++ {
		d := report[i] - report[i+1]

		isAllPositive = isAllPositive && d >= 0
		isAllNegative = isAllNegative && d < 0

		if d < 0 {
			d *= -1
		}
		isCorrectSize = isCorrectSize && (d >= 1 && d <= 3)
	}

	return isCorrectSize && (isAllPositive || isAllNegative)
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
