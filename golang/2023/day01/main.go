package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func main() {
	file, err := os.ReadFile("./input.txt")
	check(err)

	var max int = 0
	for _, s := range strings.Split(string(file), "\n\n") {
		var tot int = 0
		for _, t := range strings.Split(strings.TrimSpace(s), "\n") {
			i, err := strconv.Atoi(t)
			check(err)
			tot += i
		}
		if tot > max { max = tot }
	}

	fmt.Println(max);
}
