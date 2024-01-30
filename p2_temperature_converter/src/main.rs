/* User input
* Logic applied
* Print results
*
*
*
    Temperature in degrees Fahrenheit (°F) = (Temperature in degrees Celsius (°C) * 9/5) + 32
    Temperature in degrees Celsius (°C) = (Temperature in degrees Fahrenheit (°F) - 32) * 5/9

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
    let user_option = user_option_select();
    let temp = user_input(user_option);
    print_solution(temp);
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
fn user_input(user_option: bool) -> f32 {
    loop {
        let mut user_temp = String::new();
        if user_option == true {
            println!("Enter the temperature you wish to convert:");

            std::io::stdin()
                .read_line(&mut user_temp)
                .expect("Failed to read line");

            match user_temp.trim().parse::<f32>() {
                Ok(num) => return num * 9.0 / 5.0 + 32.0,
                Err(_) => {
                    println!("Please input a number.")
                }
            }
        } else {
            println!("Enter the temperature you wish to convert:");

            std::io::stdin()
                .read_line(&mut user_temp)
                .expect("Failed to read line");

            match user_temp.trim().parse::<f32>() {
                Ok(num) => return (num - 32.0) * 5.0 / 9.0,
                Err(_) => {
                    println!("Please input a number.")
                }
            }
        }
    }
}
fn print_solution(temp: f32) {
    println!("Solution: {}", temp);
}
