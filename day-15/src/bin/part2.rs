use std::collections::HashMap;

use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{alpha1, digit0},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
enum Operator {
    Equal,
    Dash,
}

#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    operator: Operator,
    focal_length: Option<u32>,
}

impl<'a> Lens<'a> {
    fn get_hash(&self) -> u32 {
        let nums = self
            .label
            .chars()
            .into_iter()
            .map(|c| c as u32)
            .collect::<Vec<u32>>();

        let mut current_sum: u32 = 0;
        for num in nums.iter() {
            current_sum += num;
            current_sum *= 17;
            current_sum = current_sum % 256;
        }

        current_sum
    }
}

fn main() {
    let input = include_str!("./input.txt");
    // let input = include_str!("./test.txt");
    let result = do_magic(input);
    println!("result: {}", result);
}

fn do_magic(input: &str) -> u32 {
    let (_, lenses) = parse_input(input).expect("should parse without errors");
    let mut storage_map: HashMap<u32, Vec<(&str, u32)>> = HashMap::new();

    for lens in lenses.iter() {
        let transformed_lens = (
            lens.label,
            match lens.focal_length {
                Some(n) => n,
                None => 0,
            },
        );
        let key = lens.get_hash();
        storage_map.entry(key).or_insert(vec![]);

        let existing_lenses = storage_map.get_mut(&key).unwrap();
        if let Some(index) = existing_lenses
            .iter()
            .position(|(label, _)| *label == lens.label)
        {
            match lens.operator {
                Operator::Equal => existing_lenses[index] = transformed_lens,
                Operator::Dash => {
                    existing_lenses.remove(index);
                }
            }
        } else if lens.operator == Operator::Equal {
            existing_lenses.push(transformed_lens);
        }
    }

    println!("storage_map: {:?}", storage_map);

    let result: u32 = storage_map
        .iter()
        .map(|(key, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(idx, (_, focal_length))| {
                    (key + 1) * ((idx + 1) as u32) * focal_length
                })
                .sum::<u32>()
        })
        .sum();
    result
}

fn parse_input(input: &str) -> IResult<&str, Vec<Lens>> {
    let (input, lenses) = separated_list1(tag(","), parse_lens)(input)?;

    Ok((input, lenses))
}

fn parse_lens(input: &str) -> IResult<&str, Lens> {
    let (input, (label, operator, focal_length)) = tuple((alpha1, is_a("-="), digit0))(input)?;

    let operator = match operator {
        "=" => Operator::Equal,
        "-" => Operator::Dash,
        _ => panic!("should not happen"),
    };

    let focal_length = match focal_length.parse::<u32>() {
        Ok(n) => Some(n),
        Err(_) => None,
    };

    Ok((
        input,
        Lens {
            label,
            operator,
            focal_length,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works() {
        let input = include_str!("./test.txt");
        let result = do_magic(input);
        assert_eq!(result, 145);
    }
}
