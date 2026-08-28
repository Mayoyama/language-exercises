fn ft_seed_inventory(seed_type: &str, quantity: i32, unit: &str) {
    let first_char_capitalize = seed_type
        .chars()
        .next()
        .unwrap()
        .to_uppercase()
        .to_string();

    let name = format!("{}{}", first_char_capitalize, &seed_type[1..]);

    match unit {
        "packets" => println!("{} seeds: {} packets available", name, quantity),
        "grams" => println!("{} seeds: {} grams total", name, quantity),
        "area" => println!("{} seeds: covers {} square meters", name, quantity),
        _ => println!("Unknown unit type"),
    }
}

fn main() {
    ft_seed_inventory("tomato", 15, "packets");
    ft_seed_inventory("carrot", 8, "grams");
    ft_seed_inventory("lettuce", 12, "area");
}