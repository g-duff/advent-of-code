package main

import (
	"fmt"
	"os"
	"strconv"

	"github.com/g-duff/advent-of-code/go/2024/lib"
)

func main() {
	file, err := os.ReadFile("./data/day07.input")
	check(err)

	calibrationEquations := lib.ParseToIntGrid(file)

	pt1 := solvePt1(calibrationEquations)
	fmt.Println("Part 1: ", pt1)

	pt2 := solvePt2(calibrationEquations)
	fmt.Println("Part 2: ", pt2)
}

func solvePt1(calibrationEquations [][]int) int {
	totalCalibrationResult := 0
	for _, ca := range calibrationEquations {
		if calibrationResultPt1(ca[0], ca[1], ca[2:]) {
			totalCalibrationResult += ca[0]
		}
	}
	return totalCalibrationResult
}

func calibrationResultPt1(testValue int, runningTotal int, numbers []int) bool {

	if len(numbers) == 0 {
		return testValue == runningTotal 
	}

	addResult := calibrationResultPt1(testValue, runningTotal+numbers[0], numbers[1:])
	mulResult := calibrationResultPt1(testValue, runningTotal*numbers[0], numbers[1:])
	return addResult || mulResult
}

func solvePt2(calibrationEquations [][]int) int {
	totalCalibrationResult := 0
	for _, ca := range calibrationEquations {
		if calibrationResultPt2(ca[0], ca[1], ca[2:]) {
			totalCalibrationResult += ca[0]
		}
	}
	return totalCalibrationResult
}

func calibrationResultPt2(testValue int, runningTotal int, numbers []int) bool {

	if len(numbers) == 0 {
		return testValue == runningTotal 
	}

	addResult := calibrationResultPt2(testValue, runningTotal+numbers[0], numbers[1:])
	catResult := calibrationResultPt2(testValue, cat(runningTotal, numbers[0]), numbers[1:])
	mulResult := calibrationResultPt2(testValue, runningTotal*numbers[0], numbers[1:])

	return addResult || catResult || mulResult
}

func cat(x int, y int ) int {
	myStr := strconv.Itoa(x) + strconv.Itoa(y)
	myNum, err := strconv.Atoi(myStr)
	check(err)
	return myNum
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
