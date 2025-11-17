use std::io;

fn main() {
    println!("Welcome to Delicious Bites Restaurant!");
    println!("Please enter your name:");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let name = name.trim();
    println!("\nHello, {}! We're happy to serve you today!\n", name);

    println!("Today's menu:");
    println!("1. Burger - $5");
    println!("2. Pizza - $8");
    println!("3. Salad - $4");
    println!("Please choose a meal by entering the number:");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Exiting program.");
            return;
        }
    };

    match choice {
        1 => println!("You chose Burger. Enjoy your meal, {}!", name),
        2 => println!("You chose Pizza. Enjoy your meal, {}!", name),
        3 => println!("You chose Salad. Enjoy your meal, {}!", name),
        _ => println!("Invalid choice. Please run the program again."),
    }

    println!("\nThank you for visiting Delicious Bites Restaurant!");
}
