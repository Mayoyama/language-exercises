package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	var val1, val2 int //values set inside nested code need to be declared like in C, then assigned with "=", not ":="
	var err1, err2 error

	scanner := bufio.NewScanner(os.Stdin) //":=" declares and assigns on the same line
	for {
		fmt.Print("Enter length: ")
		scanner.Scan()
		scan1 := scanner.Text()
		val1, err1 = strconv.Atoi(scan1)
		if err1 != nil {
			fmt.Println("Try again: " + err1.Error())
			continue
		}
		if val1 < 1 {
			fmt.Println("Try again: value must be positive")
			continue
		}
		break
	}
	for {
		fmt.Print("Enter width: ")
		scanner.Scan()
		scan2 := scanner.Text()
		val2, err2 = strconv.Atoi(scan2)
		if err2 != nil {
			fmt.Println("Try again: " + err1.Error())
			continue
		}
		if val2 < 1 {
			fmt.Println("Try again: value must be positive")
			continue
		}
		break
	}

	res := val1 * val2
	fmt.Printf("Plot area: %d\n", res)
}
