package main

import (
	"errors"
	"fmt"
	"os"
	"slices"
	"strings"
)

func main() {
	file, err := os.ReadFile("./data/day10.input")
	check(err)

	input := parseInput(file)

	ansPt1, ansPt2 := solve(input)
	fmt.Println("Part 1:", ansPt1)
	fmt.Println("Part 2:", ansPt2)
}

func solve(input []string) (int, int) {

	pt1, pt2 := 0, 0

	var closingChunkScores []int
	for _, line := range input {
		var stk stack
		var breakLine bool
		for _, c := range line {
			var b rune
			breakLine = false
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
					pt1 += 3
					breakLine = true
				}
			case ']':
				stk, b, _ = stk.pop()
				if b != ']' {
					pt1 += 57
					breakLine = true
				}
			case '}':
				stk, b, _ = stk.pop()
				if b != '}' {
					pt1 += 1197
					breakLine = true
				}
			case '>':
				stk, b, _ = stk.pop()
				if b != '>' {
					pt1 += 25137
					breakLine = true
				}
			}
			if breakLine {
				break
			}
		}
		if !breakLine && len(stk) != 0 {
			score := 0
			for i := len(stk) - 1; i >= 0; i-- {
				score *= 5
				switch stk[i] {
				case ')':
					score += 1
				case ']':
					score += 2
				case '}':
					score += 3
				case '>':
					score += 4
				}
			}
			closingChunkScores = append(closingChunkScores, score)
		}
	}

	slices.Sort(closingChunkScores)

	midIdx := len(closingChunkScores) / 2
	pt2 = closingChunkScores[midIdx]

	return pt1, pt2
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
