use std::io;


fn main() {
    let mut incremental_number: i32 = 0;
    loop {
        println!("Give me a number:");
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let parsed_value_result = input.trim().parse::<i32>();

        if !check_correct_value(input.trim().to_string()) && parsed_value_result.is_ok() {
            let parsed_value = parsed_value_result.expect("Error parsing Integer value");
            incremental_number += parsed_value;
        } else {
            println!("Result: {}", incremental_number);
            break
        }
    }
}

fn check_correct_value(s: String) -> bool {
    let escape_string = "Stop";
    return s.to_lowercase() == escape_string.to_lowercase()
}
