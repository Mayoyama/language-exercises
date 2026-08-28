use std::io;

fn ft_plot_area() {
	let width;
	let length;
	
	loop {
		let mut x = String::new();
		println!("Enter length: ");
		io::stdin().read_line(&mut x).expect("Failure to read input");
		
		match x.trim().parse::<i32>() {
			Ok(num) => { //.trim().parse::<i32> successfully atoi() to an int (Ok), then
				if num < 1 {
					println!("Invalid input. Try again.");
					continue;
				}
				length = num;
				break;
			}
			Err(_) => println!("Invalid input. Try again."), //Failed: results in the error print
		}
	}
	
	loop {
		let mut y = String::new();
		println!("Enter width: ");
		io::stdin().read_line(&mut y).expect("Failure to read input");
		
		match y.trim().parse::<i32>() {
			Ok(num) => {
				if num < 1 {
					println!("Invalid input. Try again.");
					continue;
				}
				width = num;
				break;
			}
			Err(_) => println!("Invalid input. Try again."),
		}
	}
	
	println!("Plot area: {}", (width * length));
}

fn main() {
	ft_plot_area();
}
