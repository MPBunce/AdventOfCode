package solution

import (
	"bufio"
	"fmt"
	"os"
)

func SolveDayThree(file *os.File) {

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		fmt.Println(line)
	}

}
