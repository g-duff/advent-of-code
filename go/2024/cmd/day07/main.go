package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	"unicode"
)

func main() {
	file, err := os.ReadFile("./data/day07.input")
	check(err)

	calibrationEquations := parse(file)

	pt1 := solvePt1(calibrationEquations)
	fmt.Println("Part 1: ", pt1)

	pt2 := solvePt2(calibrationEquations)
	fmt.Println("Part 2: ", pt2)
}

func solvePt1(calibrationEquations []CalibrationEquation) int {
	totalCalibrationResult := 0
	for _, ca := range calibrationEquations {
		if calibrationResult(ca.testValue, ca.numbers[0], ca.numbers[1:], false) {
			totalCalibrationResult += ca.testValue
		}
	}
	return totalCalibrationResult
}

func solvePt2(calibrationEquations []CalibrationEquation) int {
	totalCalibrationResult := 0
	for _, ca := range calibrationEquations {
		if calibrationResult(ca.testValue, ca.numbers[0], ca.numbers[1:], true) {
			totalCalibrationResult += ca.testValue
		}
	}
	return totalCalibrationResult
}

func calibrationResult(testValue int, runningTotal int, numbers []int, pt2 bool) bool {

	if len(numbers) == 0 {
		if testValue == runningTotal {
			return true
		} else {
			return false
		}
	}

	addResult := calibrationResult(testValue, runningTotal+numbers[0], numbers[1:], pt2)
	mulResult := calibrationResult(testValue, runningTotal*numbers[0], numbers[1:], pt2)
	res := addResult || mulResult

	if pt2 {
		myStr := strconv.Itoa(runningTotal) + strconv.Itoa(numbers[0])
		myNum, err := strconv.Atoi(myStr)
		check(err)

		res = res || calibrationResult(testValue, myNum, numbers[1:], pt2)
	}

	return res
}

type CalibrationEquation struct {
	testValue int
	numbers   []int
}

func parse(input []byte) []CalibrationEquation {
	data := strings.TrimSpace(string(input))
	lines := strings.Split(data, "\n")

	calibrationEquations := make([]CalibrationEquation, len(lines))

	for i, line := range lines {
		fields := strings.FieldsFunc(line, func(r rune) bool { return !unicode.IsNumber(r) })

		testValue, tvErr := strconv.Atoi(fields[0])
		check(tvErr)

		numbers := make([]int, len(fields)-1)
		for j, f := range fields[1:] {
			n, nErr := strconv.Atoi(f)
			check(nErr)
			numbers[j] = n
		}

		calibrationEquations[i].testValue = testValue
		calibrationEquations[i].numbers = numbers
	}

	return calibrationEquations
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
