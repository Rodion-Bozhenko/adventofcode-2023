// Package main
package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

var colors = map[string]int{
	"red":   12,
	"green": 13,
	"blue":  14,
}

func main() {
	inputFile, err := os.Open("../input.txt")
	failOnError(err)
	defer inputFile.Close()

	var result int
	var result2 int
	scanner := bufio.NewScanner(inputFile)
	for scanner.Scan() {
		line := scanner.Text()
		first := strings.Split(line, ":")
		gameID := strings.Split(first[0], " ")[1]
		cubeSets := strings.Split(first[1], ";")
		possible := true
		var maxRed int
		var maxGreen int
		var maxBlue int
		for _, set := range cubeSets {
			for _, c := range strings.Split(set, ",") {
				cSplit := strings.Split(strings.TrimSpace(c), " ")
				count, err := strconv.Atoi(cSplit[0])
				if err != nil {
					fmt.Printf("Cannot convert to in: %s %s\n", cSplit[0], err.Error())
				}
				switch cSplit[1] {
				case "red":
					if maxRed < count {
						maxRed = count
					}
				case "green":
					if maxGreen < count {
						maxGreen = count
					}
				case "blue":
					if maxBlue < count {
						maxBlue = count
					}
				}
				if colors[cSplit[1]] < count && possible {
					possible = false
				}
			}
		}
		if possible {
			id, err := strconv.Atoi(gameID)
			if err != nil {
				fmt.Printf("Cannot convert to in: %s %s\n", gameID, err.Error())
			}
			result += id
		}
		result2 += maxRed * maxGreen * maxBlue
	}

	fmt.Printf("PART 1 RESULT: %d\n", result)
	fmt.Printf("PART 2 RESULT: %d\n", result2)
}

func failOnError(err error) {
	if err != nil {
		panic(err)
	}
}
