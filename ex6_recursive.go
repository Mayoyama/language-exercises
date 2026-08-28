package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	fmt.Print("Days until harvest: ")
	scanner.Scan()
	input := scanner.Text()
	num, _ := strconv.Atoi(input)

	var countUp func(int)
	countUp = func(day int) {
		if day > num {
			fmt.Println("Harvest time!")
			return
		}
		fmt.Printf("Day %d\n", day)
        countUp(day + 1)
	}

	countUp(1)
}