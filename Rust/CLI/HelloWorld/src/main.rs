use std::io;
use std::collections::HashMap;

fn strip_string(mut input: String) -> String {
    if input.ends_with("\n") {
        input.pop();
        if input.ends_with("\r") {
            input.pop();
        }
    }
    return input
}

fn main() {
    println!("Hello, World!");
    println!("Oh hello, user, what's your name?");

    let mut username: String = String::new();
    match io::stdin().read_line(&mut username) {
        Ok(_n) => {
            username = strip_string(username)
        },
        Err(error) => {
            println!("Error Occurred: {error}")
        }
    }

    println!("Hello {username}, My name is System.");

    println!("\nPlease input a function for me to complete for you.");

    let mut options: HashMap<&str, &str> = HashMap::new();
    options.insert("A", "This is response A.");
    options.insert("B", "This is response B.");
    options.insert("C", "This is response C.");
    options.insert("D", "This is response D.");


    loop {
        let mut selected_option: String = String::new();
        match io::stdin().read_line(&mut selected_option) {
            Ok(_n) => {
                println!("Your input is: {}", selected_option.to_uppercase().as_str());

                if options.contains_key(strip_string(selected_option.to_uppercase()).as_str()) {
                    let value = options.get(strip_string(selected_option.to_uppercase()).as_str()).unwrap_or(&"None");
                    println!("Found Key!");
                    println!("{}", value.trim_matches('"'));
                    break
                } else {
                    println!("That is not a valid option, please try again!")
                }
            },
            Err(error) => {
                println!("Error Occurred: {error}")
            }
        }
    }
}
