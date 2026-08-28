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
	
	for i := 1; i <= num; i++ {
		fmt.Printf("Day %d\n", i)
	}
	fmt.Println("Harvest time!")
}