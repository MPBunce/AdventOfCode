package solution

import (
	"bufio"
	"fmt"
	"os"
)

func SolveDayFour(file *os.File) {
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		fmt.Println(line)
	}
}
