use std::io;

fn ft_count_harvest_recursive() {
    let mut input = String::new();
    
    println!("Days until harvest: ");
    io::stdin().read_line(&mut input).expect("Stdin error");
    let val = input.trim().parse::<i32>().expect("Invalid value");

    fn helper(i: i32, val: i32) {
        if i <= val {
            println!("Day {}", i);
            helper(i + 1, val);
        }
    }
    helper(1, val);
    println!("Harvest time!");
}

fn main() {
    ft_count_harvest_recursive();
}