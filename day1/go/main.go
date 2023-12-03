// Package main
package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"unicode"
)

func main() {
	//partOne()
	partTwo()
}

func partOne() {
	file, err := os.Open("../input.txt")
	failOnError(err)
	defer file.Close()

	var sum int
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		runes := []rune(line)
		i, j := 0, len(runes)-1
		var (
			leftmost  string
			rightmost string
		)
		for i <= j {
			if leftmost == "" && unicode.IsDigit(runes[i]) {
				leftmost = string(runes[i])
			} else if leftmost == "" {
				i++
			}
			if rightmost == "" && unicode.IsDigit(runes[j]) {
				rightmost = string(runes[j])
			} else if rightmost == "" {
				j--
			}
			if rightmost != "" && leftmost != "" {
				break
			}
		}
		if rightmost == "" {
			rightmost = leftmost
		}
		if leftmost == "" {
			leftmost = rightmost
		}
		callibrationVal, err := strconv.Atoi(leftmost + rightmost)
		fmt.Println(line, callibrationVal)
		failOnError(err)
		sum += callibrationVal
	}

	fmt.Printf("PART ONE SUM: %d\n", sum)
}

func partTwo() {

	file, err := os.Open("../input.txt")
	failOnError(err)
	defer file.Close()

	var sum int
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		runes := []rune(line)
		var (
			leftNum  int
			rightNum int
		)

		var callibrationVal int

		fmt.Println(line)
		for i := 0; i < len(runes); i++ {
			if unicode.IsDigit(runes[i]) {
				leftNum, _ = strconv.Atoi(string(runes[i]))
			} else {
				leftNum = lookupWord(string(runes[:i+1]))
			}
			if leftNum != 0 {
				callibrationVal += leftNum * 10
				break
			}
		}

		for i := len(runes) - 1; i >= 0; i-- {
			if unicode.IsDigit(runes[i]) {
				rightNum, _ = strconv.Atoi(string(runes[i]))
			} else {
				rightNum = lookupWord(string(runes[i:]))
			}
			if rightNum != 0 {
				callibrationVal += rightNum
				break
			}
		}

		sum += callibrationVal
	}

	fmt.Printf("PART TWO SUM: %d\n", sum)
}

func lookupWord(s string) int {
	lookupTable := map[string]int{
		"one":   1,
		"two":   2,
		"three": 3,
		"four":  4,
		"five":  5,
		"six":   6,
		"seven": 7,
		"eight": 8,
		"nine":  9,
	}

	for k := range lookupTable {
		if strings.Contains(s, k) {
			return lookupTable[k]
		}
	}

	return 0
}

func failOnError(e error) {
	if e != nil {
		log.Fatal(e)
	}
}
