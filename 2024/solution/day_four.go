package solution

import (
	"bufio"
	"fmt"
	"os"
)

const matchString = "XMAS"

var xMasOne = [3]string{"M.S", ".A.", "M.S"}
var xMasTwo = [3]string{"S.M", ".A.", "M.S"}
var xMasThree = [3]string{"M.S", ".A.", "S.M"}
var xMasFour = [3]string{"S.M", ".A.", "S.M"}

func SolveDayFour(file *os.File) {
	var charArray [][]byte
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		var temp []byte
		for _, v := range line {
			temp = append(temp, byte(v))
		}
		charArray = append(charArray, temp)
	}

	dayFourPartOne(charArray)
	dayFourPartTwo(charArray)

}

func dayFourPartOne(byteArray [][]byte) {
	var matchArry []byte
	for _, v := range matchString {
		matchArry = append(matchArry, byte(v))
	}
	fmt.Println(matchArry)
	var res int
	res += checkHorizontal(matchArry, byteArray)
	res += checkVertical(matchArry, byteArray)
	res += checkDiagonal(matchArry, byteArray)
	fmt.Println("Result:")
	fmt.Println(res)

}

func checkXMas(byteArray [][]byte) (count int) {
	rows := len(byteArray)
	if rows < 3 {
		return
	}
	cols := len(byteArray[0])
	if cols < 3 {
		return
	}

	// Check each 3x3 window in the grid
	for row := 0; row <= rows-3; row++ {
		for col := 0; col <= cols-3; col++ {
			if isXMas(byteArray, row, col) {
				count++
			}
		}
	}
	return
}

func isXMas(byteArray [][]byte, row, col int) bool {
	// Center must be 'A'
	if byteArray[row+1][col+1] != 'A' {
		return false
	}

	// Check diagonal top-left to bottom-right (\)
	topLeft := byteArray[row][col]
	bottomRight := byteArray[row+2][col+2]
	diag1Valid := (topLeft == 'M' && bottomRight == 'S') || (topLeft == 'S' && bottomRight == 'M')

	// Check diagonal top-right to bottom-left (/)
	topRight := byteArray[row][col+2]
	bottomLeft := byteArray[row+2][col]
	diag2Valid := (topRight == 'M' && bottomLeft == 'S') || (topRight == 'S' && bottomLeft == 'M')

	return diag1Valid && diag2Valid
}

func dayFourPartTwo(byteArray [][]byte) {
	result := checkXMas(byteArray)
	fmt.Println("X-MAS count:", result)
}

func checkHorizontal(matchArry []byte, byteArray [][]byte) (count int) {
	matchLen := len(matchArry)
	for _, row := range byteArray {
		// Check left-to-right
		for j := 0; j <= len(row)-matchLen; j++ {
			match := true
			for k := 0; k < matchLen; k++ {
				if row[j+k] != matchArry[k] {
					match = false
					break
				}
			}
			if match {
				count++
			}
		}

		// Check right-to-left (reverse)
		for j := 0; j <= len(row)-matchLen; j++ {
			match := true
			for k := 0; k < matchLen; k++ {
				if row[j+k] != matchArry[matchLen-1-k] {
					match = false
					break
				}
			}
			if match {
				count++
			}
		}
	}
	return
}

func checkVertical(matchArry []byte, byteArray [][]byte) (count int) {
	matchLen := len(matchArry)
	rows := len(byteArray)
	if rows == 0 {
		return
	}
	cols := len(byteArray[0])

	for col := 0; col < cols; col++ {
		// Check top-to-bottom
		for row := 0; row <= rows-matchLen; row++ {
			match := true
			for k := 0; k < matchLen; k++ {
				if byteArray[row+k][col] != matchArry[k] {
					match = false
					break
				}
			}
			if match {
				count++
			}
		}

		// Check bottom-to-top (reverse)
		for row := 0; row <= rows-matchLen; row++ {
			match := true
			for k := 0; k < matchLen; k++ {
				if byteArray[row+k][col] != matchArry[matchLen-1-k] {
					match = false
					break
				}
			}
			if match {
				count++
			}
		}
	}
	return
}

func checkDiagonal(matchArry []byte, byteArray [][]byte) (count int) {
	matchLen := len(matchArry)
	rows := len(byteArray)
	if rows == 0 {
		return
	}
	cols := len(byteArray[0])

	// Diagonal down-right (\)
	for row := 0; row <= rows-matchLen; row++ {
		for col := 0; col <= cols-matchLen; col++ {
			match := true
			for k := 0; k < matchLen; k++ {
				if byteArray[row+k][col+k] != matchArry[k] {
					match = false
					break
				}
			}
			if match {
				count++
			}
		}
	}

	// Diagonal up-left (\ reversed)
	for row := 0; row <= rows-matchLen; row++ {
		for col := 0; col <= cols-matchLen; col++ {
			match := true
			for k := 0; k < matchLen; k++ {
				if byteArray[row+k][col+k] != matchArry[matchLen-1-k] {
					match = false
					break
				}
			}
			if match {
				count++
			}
		}
	}

	// Diagonal down-left (/)
	for row := 0; row <= rows-matchLen; row++ {
		for col := matchLen - 1; col < cols; col++ {
			match := true
			for k := 0; k < matchLen; k++ {
				if byteArray[row+k][col-k] != matchArry[k] {
					match = false
					break
				}
			}
			if match {
				count++
			}
		}
	}

	// Diagonal up-right (/ reversed)
	for row := 0; row <= rows-matchLen; row++ {
		for col := matchLen - 1; col < cols; col++ {
			match := true
			for k := 0; k < matchLen; k++ {
				if byteArray[row+k][col-k] != matchArry[matchLen-1-k] {
					match = false
					break
				}
			}
			if match {
				count++
			}
		}
	}

	return
}
