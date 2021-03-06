package main

import (
	"aoc-2020/internal/stdin"
	"fmt"
	"strconv"
)

func main() {
	lines, err := stdin.ReadLinesFromFile("input.txt")
	if err != nil {
		fmt.Println("Error when reading input:", err)
		return
	}

	timestamp, err := strconv.Atoi(lines[0])
	if err != nil {
		fmt.Println("Error when parsing timestamp:", err)
		return
	}

	busIDs, err := parseBusIDs(lines[1])
	if err != nil {
		fmt.Println("Error when parsing bus IDs:", err)
		return
	}

	result := solveA(timestamp, busIDs)
	fmt.Println("Result A:", result)

	result = solveB(busIDs)
	fmt.Println("Result B:", result)
}
