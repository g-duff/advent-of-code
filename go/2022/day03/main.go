package main

import (
    "bufio"
    "fmt"
    "log"
    "strings"
    "os"
)

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}

	// pt1(file)
	pt2(file)

}

func pt1(file *os.File) {
	total := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		upTo := len(line) / 2
		firstCompartment := line[:upTo]
		secondCompartment := line[upTo:]

		for i, r := range firstCompartment {
			if (!strings.ContainsRune(firstCompartment[i+1:], r) && strings.ContainsRune(secondCompartment, r)) {
				total += runeToPriority(r)
			}
		}
	}

	fmt.Println(total);
}

func pt2(file *os.File) {
	total := 0
	var bags []string;
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		l := len(bags)
		if ( l == 0 || l == 1) {
			bags = append(bags, line);
		} else {
			for i, r := range line {
				if (!strings.ContainsRune(line[i+1:], r) && strings.ContainsRune(bags[0], r) && strings.ContainsRune(bags[1], r)) {
					total += runeToPriority(r)
				}
			}
			bags = nil
		}
	}

	fmt.Println(total);
}

func runeToPriority(r rune) int {
	const LOWER_CASE_RUNE_START = 96
	const UPPER_CASE_RUNE_START = 64
	const UPPER_CASE_PRIORITY_BONUS = 26

	if (r > LOWER_CASE_RUNE_START) {
		return (int(r) - LOWER_CASE_RUNE_START)
	} 
	return (int(r) - UPPER_CASE_RUNE_START + UPPER_CASE_PRIORITY_BONUS)
}
