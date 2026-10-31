package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	days := []string{"Day 1", "Day 2", "Day 3"}
	total := 0

	scanner := bufio.NewScanner(os.Stdin)

	for _, day := range days {
		fmt.Print(day + " harvest: ")
		scanner.Scan()
		val := scanner.Text()
		num, _ := strconv.Atoi(val)
		total += num
	}
	fmt.Printf("Total harvest: %d\n", total)
}
