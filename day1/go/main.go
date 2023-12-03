// Package main
package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"unicode"
)

func main() {
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

	fmt.Printf("SUM: %d\n", sum)
}

func failOnError(e error) {
	if e != nil {
		log.Fatal(e)
	}
}
