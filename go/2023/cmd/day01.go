package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	"unicode"
)

func main() {
	file, err := os.ReadFile("./data/day01.example")
	check(err)

	input := strings.TrimSpace(string(file))

	p1 := part1(input);
	fmt.Println("Part 1: " + strconv.Itoa(p1));
}

func part1(input string) (int) {
	var tot int = 0
	for _, s := range strings.Split(input, "\n") {
		runes := []rune(s)
		L := len(runes)

		var start_number rune
		var end_number rune
		sfound := false;
		efound := false
		for i := 0; i<L; i++ {
			sr := runes[i]
			if (unicode.IsNumber(sr) && !sfound) {
				start_number = sr
				sfound = true
			}

			er := runes[L-i-1]
			if (unicode.IsNumber(er) && !efound) {
				end_number = er
				efound = true
			}
		}

		j, err := strconv.Atoi(string(start_number) + string(end_number))
		check(err)
		tot += j
	}
	return tot
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}
