package solution

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func SolveDayThree(file *os.File) {
	//Day One
	// patternOne := `mul\([^)]*\)`
	// scanner := bufio.NewScanner(file)
	// var finds []string

	// re := regexp.MustCompile(patternOne)

	// for scanner.Scan() {
	// 	line := scanner.Text()

	// 	// Find all matches
	// 	matches := re.FindAllString(line, -1)

	// 	// Append all matches to finds
	// 	finds = append(finds, matches...)
	// }

	// solveDayThreePartOne(finds)

	//Day Two
	scanner := bufio.NewScanner(file)
	var fullInput string

	// Read all lines into one string
	for scanner.Scan() {
		fullInput += scanner.Text()
	}

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
