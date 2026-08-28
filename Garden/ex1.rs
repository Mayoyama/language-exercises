// src/main.rs

use std::io;

fn ft_garden_name() {
	let mut name = String::new();
	println!("Enter garden name: ");
	io::stdin().read_line(&mut name).expect("Failed to read");
	
	println!("Garden: {}", name.trim()); //trim() removes the extra \0 from pressing enter at the end of stdin, otherwise an extra \0 is printed
	println!("Status: Growing well!");
}

fn main() {
	ft_garden_name();
}
