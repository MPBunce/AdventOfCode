package solution

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"sort"
	"strconv"
	"strings"
)

func SolveDayOne(file *os.File) {
	var left []int
	var right []int

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		temp := strings.Fields(line)
		l, err1 := strconv.Atoi(temp[0])
		r, err2 := strconv.Atoi(temp[1])
		if err1 != nil || err2 != nil {
			fmt.Println("Error converting string to int:", err1, err2)
			continue
		}
		left = append(left, l)
		right = append(right, r)
	}
	if err := scanner.Err(); err != nil {
		fmt.Println("Error reading file:", err)
	}

	solveDayOnePartOne(left, right)
	solveDayOnePartTwo(left, right)
}

func solveDayOnePartOne(left []int, right []int) {
	sort.Slice(left, func(i, j int) bool {
		return left[i] < left[j]
	})
	sort.Slice(right, func(i, j int) bool {
		return right[i] < right[j]
	})
	var total int
	for i, _ := range left {
		diff := left[i] - right[i]
		abs := int(math.Abs(float64(diff)))
		total += abs
	}
	fmt.Println(total)
}

func solveDayOnePartTwo(left []int, right []int) {
	// Count occurrences in right list
	rightCounts := make(map[int]int)
	for _, v := range right {
		rightCounts[v]++
	}

	// Calculate similarity score
	sum := 0
	for _, v := range left {
		sum += v * rightCounts[v] // If v not in map, returns 0
	}
	fmt.Println(sum)
}
