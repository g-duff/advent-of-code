package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	file, err := os.ReadFile("./data/day03.input")
	check(err)

	input := strings.Split(strings.TrimSpace(string(file)), "\n")

	pt1 := solvePt1(input)
	fmt.Println("Part 1:", pt1)

	pt2 := solvePt2(input)
	fmt.Println("Part 2:", pt2)
}

func solvePt1(input []string) int {
	R := len(input)
	C := len(input[0])

	counts := make([]int, C)

	for _, row := range input {
		for j, c := range row {
			x, err := strconv.Atoi(string(c))
			check(err)
			counts[j] += x
		}
	}

	gamma := 0
	epsilon := 0
	for i, c := range counts {
		if c*2 > R {
			gamma += 1 << (C - i - 1)
		} else {
			epsilon += 1 << (C - i - 1)
		}
	}

	return gamma * epsilon
}

func solvePt2(input []string) int64 {
	oxygenGeneratorRating := getOxygenGeneratorRating(input)
	CO2GeneratorRating := getCO2ScrubberRating(input)
	return oxygenGeneratorRating * CO2GeneratorRating
}

func getOxygenGeneratorRating(input []string) int64 {
	nRows := len(input)
	nCols := len(input[0])

	rowIsSuitable := make([]bool, nRows)
	for i := range rowIsSuitable {
		rowIsSuitable[i] = true
	}

	var binAns string

	for ci := 0; ci < nCols; ci++ {
		nBitsSet := 0

		for ri := 0; ri < nRows; ri++ {
			if rowIsSuitable[ri] && input[ri][ci] == byte('1') {
				nBitsSet++
			}
		}

		var dominantChar byte
		if nBitsSet*2 >= countTrue(rowIsSuitable) {
			dominantChar = byte('1')
		} else {
			dominantChar = byte('0')
		}

		for ri := 0; ri < nRows; ri++ {
			if rowIsSuitable[ri] && input[ri][ci] != dominantChar {
				rowIsSuitable[ri] = false
			}
		}

		if countTrue(rowIsSuitable) == 1 {
			for ri, b := range rowIsSuitable {
				if b {
					binAns = input[ri]
				}
			}
			break
		}
	}

	n, err := strconv.ParseInt(binAns, 2, 64)
	check(err)
	return n
}

func getCO2ScrubberRating(input []string) int64 {
	nRows := len(input)
	nCols := len(input[0])

	rowIsSuitable := make([]bool, nRows)
	for i := range rowIsSuitable {
		rowIsSuitable[i] = true
	}

	var binAns string

	for ci := 0; ci < nCols; ci++ {
		nBitsSet := 0
		for ri := 0; ri < nRows; ri++ {
			if rowIsSuitable[ri] && input[ri][ci] == byte('1') {
				nBitsSet++
			}
		}

		var leastCommonChar byte
		if nBitsSet*2 < countTrue(rowIsSuitable) {
			leastCommonChar = byte('1')
		} else {
			leastCommonChar = byte('0')
		}

		for ri := 0; ri < nRows; ri++ {
			if rowIsSuitable[ri] && input[ri][ci] != leastCommonChar {
				rowIsSuitable[ri] = false
			}
		}

		if countTrue(rowIsSuitable) == 1 {
			for ri, b := range rowIsSuitable {
				if b {
					binAns = input[ri]
				}
			}
			break
		}
	}

	n, err := strconv.ParseInt(binAns, 2, 64)
	check(err)
	return n
}

func countTrue(bools []bool) int {
	count := 0
	for _, b := range bools {
		if b {
			count++
		}
	}
	return count
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
