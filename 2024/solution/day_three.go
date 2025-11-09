package solution

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
)

func SolveDayThree(file *os.File) {
	// Read the entire file once
	scanner := bufio.NewScanner(file)
	var fullInput string

	for scanner.Scan() {
		fullInput += scanner.Text()
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	// Part One - extract mul() patterns
	patternOne := `mul\([^)]*\)`
	re := regexp.MustCompile(patternOne)
	finds := re.FindAllString(fullInput, -1)
	solveDayThreePartOne(finds)

	// Part Two - use the full input
	solveDayThreePartTwo(fullInput)
}

func solveDayThreePartOne(finds []string) {
	patternOne := `mul\((\d+),(\d+)\)`
	res := 0 // Need := for declaration
	re := regexp.MustCompile(patternOne)

	for _, v := range finds {
		// Find the numbers in each match
		match := re.FindStringSubmatch(v)

		if len(match) > 2 {
			// Parse the two numbers
			num1, err1 := strconv.Atoi(match[1])
			num2, err2 := strconv.Atoi(match[2])

			if err1 == nil && err2 == nil {
				res += num1 * num2
			}
		}
	}

	fmt.Println(res)
}

func solveDayThreePartTwo(input string) {
	// Find all relevant instructions in order: do(), don't(), and mul(x,y)
	pattern := `do\(\)|don't\(\)|mul\((\d+),(\d+)\)`
	re := regexp.MustCompile(pattern)

	matches := re.FindAllStringSubmatch(input, -1)

	enabled := true // mul starts enabled
	res := 0

	for _, match := range matches {
		instruction := match[0]

		if instruction == "do()" {
			enabled = true
		} else if instruction == "don't()" {
			enabled = false
		} else if enabled && len(match) > 2 {
			// It's a mul(x,y) and we're enabled
			num1, err1 := strconv.Atoi(match[1])
			num2, err2 := strconv.Atoi(match[2])

			if err1 == nil && err2 == nil {
				res += num1 * num2
			}
		}
	}

	fmt.Println(res)
}
