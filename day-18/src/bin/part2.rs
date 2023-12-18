

use std::{iter::repeat};

use glam::I64Vec2;
use itertools::Itertools;
use nom::{
    bytes::complete::{take, take_until},
    character::complete::{self},
    multi::many1,
    sequence::terminated,
    IResult,
};
use tracing::{info, span, Level};

#[derive(Debug)]
struct Instruction {
    direction: I64Vec2,
    amount: i64,
}


fn main() {
    let input = include_str!("./input.txt");
    // let input = include_str!("./test.txt");
    let result = do_magic(input);
    println!("result: {}", result);
}

fn do_magic(input: &str) -> i64 {
    let (_, instructions) = parse_grid(input).expect("should never fail");

    println!("instructions: {:?}", instructions);

    let vertices = instructions
        .iter()
        .scan(I64Vec2::new(0, 0), |state, next| {
            *state += next.direction * next.amount;
            Some(*state)
        })
        .collect::<Vec<I64Vec2>>();

    let perimeter_length = vertices
        .iter()
        .tuple_windows()
        .map(|(one, two)| {
            let distance = (*two - *one).abs();
            distance.x + distance.y
        })
        .sum::<i64>()
        + {
            let one = vertices.iter().last().unwrap();
            let two = vertices.iter().next().unwrap();
            let distance = (*two - *one).abs();
            distance.x + distance.y
        };
    let area = ((vertices
        .iter()
        .tuple_windows()
        .map(|(one, two)| one.x * two.y - one.y * two.x)
        .sum::<i64>()
        + perimeter_length)
        / 2)
    .abs()
        + 1;

    perimeter_length  + area
}

fn parse_grid(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = many1(instruction)(input)?;

    Ok((input, instructions))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
  let (input, _) = terminated(
    take_until("#"),
    complete::char('#'),
)(input)?;
let (input, hex) = take(5usize)(input)?;
let (input, direction) = take(1usize)(input)?;

let amount =
    i64::from_str_radix(hex, 16).expect("should parse");
let direction = match i64::from_str_radix(direction, 16)
    .expect("should parse")
{
    0 => I64Vec2::X,
    1 => I64Vec2::NEG_Y,
    2 => I64Vec2::NEG_X,
    3 => I64Vec2::Y,
    _ => unreachable!("should never happen"),
};

Ok((
    input,
    Instruction { direction, amount },
))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./test.txt");
        let result = do_magic(input);
        assert_eq!(result, 952408144115);
    }
}
