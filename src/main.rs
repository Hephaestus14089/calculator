use std::env::args;

fn main() {
    let mut args = args();
    // println!("{:?}", args);

    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number: f32 = second.parse().unwrap();
    let result = operate(operator, first_number, second_number);
    
    println!("{:?}", output(first_number, second_number, operator, result));
} // end of main()

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        '/' => first_number / second_number,
        _ => panic!("Invalid operator")
    }
} // end of operate function

fn output(first_number: f32, second_number: f32, operator: char, result: f32) -> String {
    return format!("{} {} {} = {}", first_number, operator, second_number, result);
} // end of output function
