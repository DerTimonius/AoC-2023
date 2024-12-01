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
    let mut num_vec: Vec<char> = Vec::new();
    let chars = input.chars();
    chars.for_each(|char| {
        if char.is_digit(10) {
            num_vec.push(char)
        }
    });
    let first_digit: String;
    let second_digit: String;
    match num_vec.len() {
        0 => {
            first_digit = "0".to_string();
            second_digit = "0".to_string();
        }
        _ => {
            first_digit = num_vec[0].to_string();
            second_digit = num_vec[num_vec.len() - 1].to_string();
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
            "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet",
        );
        assert_eq!(result, 142);
    }
}
