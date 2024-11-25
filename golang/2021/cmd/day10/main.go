package main

import (
	"errors"
	"fmt"
	"os"
	"strings"
)

func main() {
	file, err := os.ReadFile("./data/day10.example")
	check(err)

	input := parseInput(file)

	ansPt1 := solvePt1(input)
	fmt.Println("Part 1:", ansPt1)

	ansPt2 := solvePt2(input)
	fmt.Println("Part 2:", ansPt2)
}

func solvePt1(input []string) int {

	ans := 0

	for _, line := range input {
		var stk stack
		for _, c := range line {
			var b rune
			var breakLine = false
			switch c {
			case '(':
				stk = stk.push(')')
			case '[':
				stk = stk.push(']')
			case '{':
				stk = stk.push('}')
			case '<':
				stk = stk.push('>')
			case ')':
				stk, b, _ = stk.pop()
				if b != ')' {
					ans += 3
					breakLine = true
				}
			case ']':
				stk, b, _ = stk.pop()
				if b != ']' {
					ans += 57
					breakLine = true
				}
			case '}':
				stk, b, _ = stk.pop()
				if b != '}' {
					ans += 1197
					breakLine = true
				}
			case '>':
				stk, b, _ = stk.pop()
				if b != '>' {
					ans += 25137
					breakLine = true
				}
			}
			if breakLine {
				break
			}
		}
	}
	return ans
}

func solvePt2(input []string) int {
	return 0
}

func parseInput(file []byte) []string {
	input := string(file)

	parsed := strings.Split(strings.TrimSpace(input), "\n")

	return parsed
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

type stack []rune

func (s stack) push(r rune) stack {
	return append(s, r)
}

func (s stack) pop() (stack, rune, error) {
	l := len(s)
	if l > 0 {
		return s[:l-1], s[l-1], nil
	} else {
		return s, '0', errors.New("emptyStack")
	}
}
