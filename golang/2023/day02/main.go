package main

import (
	"fmt"
	"os"
	"bufio"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	pt1();
	pt2();
}

func pt1() {
	file, err := os.Open("./input.txt")
	check(err)

	totalScore := 0;
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		round := strings.Split(scanner.Text(), " ");
		away, home := round[0], round[1]
		var homeVal int;
		var awayVal int;

		switch {
		case away == "A": awayVal = 1
		case away == "B": awayVal = 2
		case away == "C": awayVal = 3
		}

		switch {
		case home == "X": homeVal = 1
		case home == "Y": homeVal = 2
		case home == "Z": homeVal = 3
		}

		roundScore := 0;
		roundScore = homeVal;

		switch {
		case homeVal == awayVal: roundScore += 3
		case homeVal == 1 && awayVal == 3: roundScore += 6
		case homeVal == 2 && awayVal == 1: roundScore += 6
		case homeVal == 3 && awayVal == 2: roundScore += 6
		}

		totalScore += roundScore
	}

	fmt.Println(totalScore)
}



func pt2() {
	file, err := os.Open("./input.txt")
	check(err)

	// Row: rock, paper, scisors
	// Col: lose, draw, win
	scoreMatrix := [3][3]int{
		{3, 1 + 3, 2 + 6},
		{1, 2 + 3, 3 + 6},
		{2, 3 + 3, 1 + 6},
	}

	totalScore := 0;
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		round := strings.Split(scanner.Text(), " ");
		away, home := round[0], round[1]
		var homeVal int;
		var awayVal int;

		switch {
		case away == "A": awayVal = 0
		case away == "B": awayVal = 1
		case away == "C": awayVal = 2
		}

		switch {
		case home == "X": homeVal = 0
		case home == "Y": homeVal = 1
		case home == "Z": homeVal = 2
		}

		totalScore += scoreMatrix[awayVal][homeVal]
	}

	fmt.Println(totalScore)
}
