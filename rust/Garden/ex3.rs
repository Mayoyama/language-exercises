use std::io;

fn ft_harvest_total() {
	let mut total = 0;
	

	for i in 1..=3 {
		let mut input = String::new();
		println!("Day {} harvest: ", i);
		io::stdin().read_line(&mut input).expect("Failure to read input");
		total += input.trim().parse::<i32>().unwrap_or(0);
	}
	
	
	println!("Total harvest: {}", total);
}

fn main() {
	ft_harvest_total();
}
