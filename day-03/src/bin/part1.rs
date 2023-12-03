#[derive(Debug)]
struct Position(i32, i32);

fn main() {
    let input = include_str!("./input.txt");
    // let input = include_str!("./test.txt");
    let result = do_magic(input);
    println!("result: {}", result);
}

fn do_magic(input: &str) -> u32 {
    let lines = input.lines();
    let row_numbers = lines.count();
    let col_numbers = input.lines().next().unwrap().chars().count();
    let mut numbers: Vec<(&str, Position)> = vec![];
    let mut symbols: Vec<(char, Position)> = vec![];
    let number_regex = regex::Regex::new(r"\d+").unwrap();
    for (row, line) in input.lines().enumerate() {
        for matched in number_regex.find_iter(line) {
            numbers.push((
                matched.as_str(),
                Position(row as i32, matched.start() as i32),
            ));
        }
        for (col, char) in line.chars().enumerate() {
            if !char.is_ascii_digit() && char != '.' {
                symbols.push((char, Position(row as i32, col as i32)));
            }
        }
    }
    let mut part_numbers: Vec<u32> = vec![];
    for (number_str, number_start_pos) in numbers.iter() {
        let number_end_pos = Position(number_start_pos.0, number_start_pos.1 + number_str.len() as i32 - 1);
        for (_, symbol_pos) in symbols.iter() {
            let neighbors = [
                Position(symbol_pos.0 - 1, symbol_pos.1 - 1),
                Position(symbol_pos.0 - 1, symbol_pos.1),
                Position(symbol_pos.0 - 1, symbol_pos.1 + 1),
                Position(symbol_pos.0, symbol_pos.1 - 1),
                Position(symbol_pos.0, symbol_pos.1 + 1),
                Position(symbol_pos.0 + 1, symbol_pos.1 - 1),
                Position(symbol_pos.0 + 1, symbol_pos.1),
                Position(symbol_pos.0 + 1, symbol_pos.1 + 1),
            ]
            .into_iter()
            .filter(|p| {
                p.0 >= 0 && p.1 >= 0 && p.0 < row_numbers as i32 && p.1 < col_numbers as i32
            })
            .collect::<Vec<Position>>();
            for neighbor in neighbors.iter() {
                if neighbor.0 == number_start_pos.0
                && neighbor.1 >= number_start_pos.1
                && neighbor.1 <= number_end_pos.1
                {
                    part_numbers.push(number_str.parse::<u32>().unwrap());
                    break;
                }
            }
        }
    }
    part_numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./test.txt");
        let result = do_magic(input);
        assert_eq!(result, 4361);
    }
}
