package main

import (
	"fmt"
	"os"
	"bufio"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	fmt.Print("Enter garden name: ")
	scanner.Scan()
	input := scanner.Text()
	fmt.Printf("Garden: %s\nStatus: Growing Well!\n", input)
}