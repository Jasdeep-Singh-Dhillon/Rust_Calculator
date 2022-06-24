use std::io::Write;



fn main() {
    let input = get_input("Enter integer: ");

    match input {
        Some(n) => println!("You Entered {}", n),
        None => {}
    }
}

fn get_input(prompt: &str) -> Option<i32> {
    print!("{}", prompt);
    let _ = std::io::stdout().flush();
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");
    
    let trimmed = input.trim();
    
    match trimmed.parse::<i32>() {
        Ok(n) => Some(n),
        Err(..) => {
            println!("Please enter an integer, {} is not an Integer", trimmed);
            None
        }
    }
}