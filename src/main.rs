use std::io::Write;
 fn main() {

    loop {
        let option: i32 = get_integer("
        1. Addition
        2. Subtraction
        3. Multiplication
        4. Division
        5. Quit
        Select Operation:
        ");
        
        if option < 1 || option > 5 {
            println!("Enter a valid option [1-5]. Try again");
            continue;
        }

        if option == 5 {
            break;
        }

        let nums: (i32, i32) = (
            get_integer("Enter Number 1: "),
            get_integer("Enter number 2: ")
        );
        
        if option == 1 {
            println!("{} + {} = {}", nums.0, nums.1, nums.0 + nums.1);
        } else if option == 2 {
            println!("{} - {} = {}", nums.0, nums.1, nums.0 - nums.1);
        } else if option == 3 {
            println!("{} * {} = {}", nums.0, nums.1, nums.0 * nums.1);
        } else if option == 4 {
            println!("{} / {} = {:.3}", nums.0, nums.1, (nums.0 as f32) / (nums.1 as f32));
        } 
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    let _ = std::io::stdout().flush();
    let mut input: String = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");
    input
}

fn get_integer(prompt: &str) -> i32 {
    let input: String = get_input(prompt);
    let number: i32 = input.trim().parse().expect("Please enter a valid integer");

    number
}