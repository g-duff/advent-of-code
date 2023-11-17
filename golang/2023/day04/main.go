package main

import (
	"strconv"
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	file, err := os.Open("./input.txt")
	if err != nil {
		log.Fatal(err)
	}

	count := 0
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		pairs := strings.Split(line, ",")
		firstPair, secondPair := strings.Split(pairs[0], "-"), strings.Split(pairs[1], "-")

		f1, _ := strconv.Atoi(firstPair[0])
		f2, _ := strconv.Atoi(firstPair[1])
		s1, _ := strconv.Atoi(secondPair[0])
		s2, _ := strconv.Atoi(secondPair[1])

		if (f1 <= s1 && f2 >= s2) || (f1 >= s1 && f2 <= s2) {
			fmt.Println(firstPair, secondPair)
			count++
		}
	}

	fmt.Println(count)
}

