package main

import (
	"AdventOfCode/2024/solution" // Change 'yourmodule' to your actual module name
	"flag"
	"fmt"
	"log"
	"os"
)

func main() {
	var useSampleInput = flag.Bool("sampleText", false, "a bool")
	flag.Parse()
	day := "day_five" // or get from args/env

	inputPath := fmt.Sprintf("./input/%s.txt", day)
	if *useSampleInput {
		inputPath = "./input/sample.txt"
	}
	file, err := os.Open(inputPath)
	if err != nil {
		log.Fatalf("failed to open input file: %v", err)
	}
	defer file.Close()

	switch day {
	case "day_one":
		solution.SolveDayOne(file)
	case "day_two":
		solution.SolveDayTwo(file)
	case "day_three":
		solution.SolveDayThree(file)
	case "day_four":
		solution.SolveDayFour(file)
	case "day_five":
		solution.SolveDayFive(file)
	// case "day_six":
	// 	solution.SolveDaySix(file)
	// case "day_seven":
	// 	solution.SolveDaySeven(file)
	// case "day_eight":
	// 	solution.SolveDayEight(file)
	// case "day_nine":
	// 	solution.SolveDayNine(file)
	// case "day_ten":
	// 	solution.SolveDayTen(file)
	// case "day_eleven":
	// 	solution.SolveDayEleven(file)
	// case "day_twelve":
	// 	solution.SolveDayTwelve(file)
	// case "day_thirteen":
	// 	solution.SolveDayThirteen(file)
	// case "day_fourteen":
	// 	solution.SolveDayFourteen(file)
	// case "day_fifteen":
	// 	solution.SolveDayFifteen(file)
	// case "day_sixteen":
	// 	solution.SolveDaySixteen(file)
	// case "day_seventeen":
	// 	solution.SolveDaySeventeen(file)
	// case "day_eighteen":
	// 	solution.SolveDayEighteen(file)
	// case "day_nineteen":
	// 	solution.SolveDayNineteen(file)
	// case "day_twenty":
	// 	solution.SolveDayTwenty(file)
	// case "day_twenty_one":
	// 	solution.SolveDayTwentyOne(file)
	// case "day_twenty_two":
	// 	solution.SolveDayTwentyTwo(file)
	// case "day_twenty_three":
	// 	solution.SolveDayTwentyThree(file)
	// case "day_twenty_four":
	// 	solution.SolveDayTwentyFour(file)
	// case "day_twenty_five":
	// 	solution.SolveDayTwentyFive(file)
	default:
		log.Fatalf("unknown day: %s", day)
	}
}
