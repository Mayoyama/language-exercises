use std::io;

fn ft_water_reminder() {
    let mut input = String::new();

    println!("Days since last watering: ");
    io::stdin().read_line(&mut input).expect("Stdin error");

    match input.trim().parse::<i32>() {
        Ok(val) => {
            println!("Days since last watering: {}", val);

            if val > 2 {
                println!("Water the plants!");
            } else {
                println!("Plants are fine");
            }
        }
        Err(_) => println!("Error converting value to int"),
    }
}

fn main() {
    ft_water_reminder();
}