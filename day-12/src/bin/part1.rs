use itertools::Itertools;
use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Spring {
    Unknown,
    Damaged,
    Operational
}

#[derive(Debug)]
struct Record {
    springs: Vec<Spring>,
    numbers: Vec<usize>,
}

impl Record {
    fn is_valid(&self) -> bool {
        self.springs
            .iter()
            .group_by(|item| *item)
            .into_iter()
            .filter_map(|(key, group)| {
                if *key == Spring::Damaged {
                    Some(group.count())
                } else {
                    None
                }
            })
            .eq(self.numbers.iter().copied())
    }

    fn valid_arrangements(&self) -> usize {
        if let Some(index) = self
            .springs
            .iter()
            .position(|spring| *spring == Spring::Unknown)
        {
            // treat unknown spring as damaged
            let mut as_damaged_spring = self.springs.clone();
            as_damaged_spring[index] = Spring::Damaged;
            let as_damaged = Record {
                springs: as_damaged_spring,
                numbers: self.numbers.to_vec(),
            };

            // treat unknown spring as operational
            let mut as_operational_spring = self.springs.clone();
            as_operational_spring[index] = Spring::Operational;
            let as_operational = Record {
                springs: as_operational_spring,
                numbers: self.numbers.to_vec(),
            };

            as_damaged.valid_arrangements() + as_operational.valid_arrangements()
        } else {
            if self.is_valid() {
                1
            } else {
                0
            }
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    // let input = include_str!("./test.txt");
    let result = do_magic(input);
    println!("result: {}", result);
}

fn do_magic(input: &str) -> usize {
    let (_, records) = parse_input(input).expect("should parse correctly");

    let sum = records.iter().map(|record| record.valid_arrangements()).sum();
    sum
}

fn parse_input(input: &str) -> IResult<&str, Vec<Record>> {
    let (input, records) = separated_list1(line_ending, parse_record)(input)?;

    Ok((input, records))
}

fn parse_record(input: &str) -> IResult<&str, Record> {
    let (input, (springs, numbers)) = separated_pair(
        is_a(".#?"),
        space1,
        separated_list1(tag(","), complete::u32),
    )(input)?;

    let springs = springs.chars().map(|c| match c {
        '#' => Spring::Damaged,
        '.' => Spring::Operational,
        '?' => Spring::Unknown,
        _ => panic!("encountered unexpected character")
    }).collect();

    let numbers = numbers.iter().map(|num| *num as usize).collect();

    Ok((
        input,
        Record {
            springs,
            numbers,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = include_str!("./test.txt");
        let result = do_magic(input);
        assert_eq!(result, 21);
    }
}
