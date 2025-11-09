package solution

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func SolveDayTwo(file *os.File) {
	var arr [][]int
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		tokens := strings.Fields(line)
		var row []int
		for _, t := range tokens {
			num, err := strconv.Atoi(t)
			if err != nil {
				fmt.Println("Error converting:", t, err)
				continue
			}
			row = append(row, num)
		}
		arr = append(arr, row)
	}
	if err := scanner.Err(); err != nil {
		fmt.Println("Error reading file:", err)
	}
	//fmt.Println(arr)

	solvePartOne(arr)
	solvePartTwo(arr)
}

func solvePartOne(arr [][]int) {
	count := 0
	for _, v := range arr {
		count += validateRow(v, v[0] > v[1])
	}
	fmt.Println(count)
}

func validateRow(row []int, isDecrease bool) int {
	for i := 0; i < len(row)-1; i++ {
		if isDecrease {
			diff := row[i] - row[i+1]
			if diff > 3 || diff < 1 {
				return 0
			}
		} else {
			diff := row[i+1] - row[i]
			if diff > 3 || diff < 1 {
				return 0
			}
		}

	}
	return 1
}

func solvePartTwo(arr [][]int) {
	count := 0
	for _, v := range arr {
		count += validateRowTwo(v)
	}
	fmt.Println("PART TWO:")
	fmt.Println(count)
}

func validateRowTwo(row []int) int {
	// First check if already safe without removing anything
	if validateRow(row, row[0] > row[1]) == 1 {
		return 1
	}

	// Try removing each element position (0 to len-1)
	for i := 0; i < len(row); i++ {
		// Create a NEW slice without element at index i
		slice := make([]int, 0, len(row)-1)
		slice = append(slice, row[:i]...)
		slice = append(slice, row[i+1:]...)

		// Check if this works (let it determine its own direction)
		if len(slice) >= 2 && validateRow(slice, slice[0] > slice[1]) == 1 {
			return 1
		}
	}

	return 0
}
