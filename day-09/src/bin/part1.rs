use nom::{
    character::complete::{self, multispace1},
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = include_str!("./input.txt");
    // let input = include_str!("./test.txt");
    let result = do_magic(input);
    println!("result: {}", result);
}

fn do_magic(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let numbers = lines
        .iter()
        .map(|line| {
            let (_, numbers) = parse_line(line).expect("should parse");
            numbers
        })
        .collect::<Vec<Vec<i32>>>();

    numbers.iter().map(|nums| get_next_value(nums)).sum()
}

fn parse_line(input: &str) -> IResult<&str, Vec<i32>> {
    let (input, numbers) = separated_list1(multispace1, complete::i32)(input)?;

    Ok((input, numbers))
}

fn get_next_value(numbers: &Vec<i32>) -> i32 {
    let mut next = numbers[numbers.len() - 1];
    let mut calc = numbers.clone();

    loop {
        let mut row_difference = Vec::new();

        for (index, num) in calc.iter().enumerate() {
            if index == 0 {
                continue;
            }
            row_difference.push(num - calc[index - 1])
        }

        if row_difference.iter().all(|num| *num == 0) {
            break;
        }

        next += row_difference.last().unwrap();

        calc = row_difference;
    }

    next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./test.txt");
        let result = do_magic(input);
        assert_eq!(result, 114);
    }
}
