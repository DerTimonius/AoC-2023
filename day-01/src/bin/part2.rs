use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let result = calibrate(input);
    println!("{:?}", result);
}
fn calibrate(input: &str) -> usize {
    let result: usize = input
        .split("\n")
        .into_iter()
        .map(|line| extract_number(line))
        .sum();
    result
}

fn extract_number(input: &str) -> usize {
    let valid_digits = HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    let mut num_vec: Vec<String> = Vec::new();
    let mut curr_word = String::new();
    for char in input.chars() {
        curr_word.push(char);
        if char.is_digit(10) {
            num_vec.push(char.to_string());
            curr_word.clear()
        }
        if let Some(x) = valid_digits.get(curr_word.as_str()) {
            num_vec.push(x.to_string());
        } else if curr_word.len() > 3 {
            let mut temp_word = curr_word.clone();
            while temp_word.len() >= 3 {
                if let Some(x) = valid_digits.get(temp_word.as_str()) {
                    num_vec.push(x.to_string());
                }
                temp_word.remove(0);
            }
        }
    }
    let first_digit: String;
    let second_digit: String;
    match num_vec.len() {
        0 => {
            first_digit = "0".to_string();
            second_digit = "0".to_string();
        }
        _ => {
            first_digit = num_vec[0].clone();
            second_digit = num_vec[num_vec.len() - 1].clone();
        }
    }
    let num = format!("{}{}", first_digit, second_digit)
        .parse::<usize>()
        .unwrap();
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = calibrate(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
            ",
        );
        assert_eq!(result, 281);
    }
}
