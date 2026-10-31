use std::io;

fn  ft_plant_age() {

    let mut input = String::new();
    println!("Enter plant age in days: ");
    io::stdin().read_line(&mut input).expect("Failed");

    match input.trim().parse::<i32>() {
        Ok(val) => {
            if val > 60 {
                println!("Plant is ready to harvest!");
            } else {
            println!("Plant needs more time to grow.");
            }
        }
        Err(_) => println!("Invalid input."),
    }
}

fn main() {
    ft_plant_age();
}