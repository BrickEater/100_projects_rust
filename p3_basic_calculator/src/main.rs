fn main() {
    instructions();
    let operand1 = input_operand();
    let operator = input_operator();
    let operand2 = input_operand();
    let solution = calculate(operand1, operator, operand2);
    println!("Solution: {}", solution);
}

fn instructions() {
    println!("Enter the terms for a basic calculation")
}
fn input_operand() -> f32 {
    loop {
        let mut user_input = String::new();
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        match user_input.trim().parse::<f32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter a number");
            }
        }
    }
}
fn input_operator() -> String {
    loop {
        let mut user_input = String::new();
        std::io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        match user_input.trim() {
            "+" => return,
            "-" => return,
            "*" => return,
            "/" => return,
            _ => {
                println!("Please enter an a +, -, *, /");
            }
        }
    }
}
fn calculate(oerpand1: f32, operator: String, operand2: f32) -> f32 {}
