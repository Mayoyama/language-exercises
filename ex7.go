package main

import (
	"fmt"
	"strings"
)

func ft_seed_inventory(plant string, amount int, unit string) {
	capitalize := func(name string) string {
		if name[0] >= 'a' && name[0] <= 'z' {
			return (strings.ToUpper(string(name[0])) + strings.ToLower(name[1:]))
		}
		return name
	}

	if unit == "packets" {
		fmt.Printf(capitalize(plant)+" seeds: %d %s available\n", amount, unit)
	} else if unit == "grams" {
		fmt.Printf(capitalize(plant)+" seeds: %d %s total\n", amount, unit)
	} else if unit == "area" {
		fmt.Printf(capitalize(plant)+" seeds: covers %d square meters\n", amount)
	} else {
		fmt.Println("Unknown unit type")
	}

}

func main() {
	ft_seed_inventory("tomato", 15, "packets")
	ft_seed_inventory("carrot", 8, "grams")
	ft_seed_inventory("lettuce", 12, "area")
}
