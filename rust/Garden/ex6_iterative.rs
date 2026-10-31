use std::io;

fn ft_count_harvest_iterative() {
    let mut input = String::new();

    println!("Days until harvest: ");
    io::stdin().read_line(&mut input).expect("Stdin error");

    let val = input.trim().parse::<i32>().expect("Invalid value");

    for i in 1..=val {
        println!("Day {}", i);
    }
    println!("Harvest time!");
}