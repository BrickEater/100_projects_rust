/* User input
* Logic applied
* Print results
*
*
*
*
*
*
*
*
*
*
*/

fn main() {
    print_instructions();
    user_option_select();
    user_input();
    calculate_conversion();
    print_solution();
}

fn print_instructions() {
    println!(
        "Convert temperatures\n\
        between Celsius and Fahrenheit"
    )
}
fn user_option_select() -> bool {
    loop {
        let mut user_option = String::new();
        println!(
            "Select between:\n\
            1. Celsius to Fahrenheit\n\
            2. Fahrenheit to Celsius"
        );
        std::io::stdin()
            .read_line(&mut user_option)
            .expect("Failed to read line");

        match user_option.trim() {
            "1" => {
                println!("Celsius to Fahrenheit");
                return true;
            }
            "2" => {
                println!("Fahrenheit to Celsius");
                return false;
            }
            _ => {
                println!("Invalid choice.");
                continue;
            }
        }
    }
}
fn user_input() {}
fn calculate_conversion() {}
fn print_solution() {}
