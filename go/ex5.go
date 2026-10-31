package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	fmt.Print("Days since last watering: ")
	scanner.Scan()
	input := scanner.Text()
	num, _ := strconv.Atoi(input)
	if num > 2 {
		fmt.Println("Water the plants!")
	} else {
		fmt.Println("Plants are fine")
	}
}
