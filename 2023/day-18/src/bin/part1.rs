use std::{iter::repeat, ops::RangeInclusive};

use glam::IVec2;
use itertools::{Itertools, MinMaxResult};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, hex_digit1, line_ending, space1},
    multi::separated_list1,
    sequence::delimited,
    IResult, Parser,
};

use tracing::{span, Level};

#[derive(Debug)]
struct Instruction<'a> {
    direction: IVec2,
    amount: i32,
    hex_code: &'a str,
}

fn main() {
    // let input = include_str!("./input.txt");
    let input = include_str!("./test.txt");
    let result = do_magic(input);
    println!("result: {}", result);
}

fn do_magic(input: &str) -> usize {
    let (_, instructions) = parse_grid(input).expect("should never fail");

    let travel = instructions
        .iter()
        .flat_map(|instruction| repeat(instruction).take(instruction.amount as usize))
        .scan(IVec2::new(0, 0), |state, next| {
            *state += next.direction;

            Some(*state)
        })
        .collect::<Vec<IVec2>>();

    let MinMaxResult::MinMax(x_min, x_max) = travel.iter().map(|pos| pos.x).minmax() else {
        panic!("should have a min and max for x");
    };

    let MinMaxResult::MinMax(y_min, y_max) = travel.iter().map(|pos| pos.y).minmax() else {
        panic!("should have a min and max for y");
    };

    print_grid(&travel, y_min..=y_max, x_min..=x_max);

    let interior_tiles = (y_min..=y_max)
        .map(|row| {
            ((x_min - 1)..x_max).fold(
                (0, None::<IVec2>, vec![]),
                |(mut crossings, mut last_crossing, mut all_interior_tiles), next_position| {
                    let next_ivec = IVec2::new(next_position, row);
                    let my_span = span!(
                        Level::INFO,
                        "row_span",
                        crossings,
                        ?last_crossing,
                        ?next_ivec,
                        row
                    );
                    my_span.in_scope(|| {
                        match travel.contains(&next_ivec) {
                            true => {
                                if last_crossing.is_none() {
                                    crossings += 1;
                                    last_crossing = Some(next_ivec);
                                }
                            }
                            false => {
                                if last_crossing
                                    .is_some_and(|cross| (next_ivec - cross) == IVec2::new(1, 0))
                                {
                                    // if we land on an empty square
                                    // and the last crossing is directly
                                    // before the empty space, then reset
                                    // last_crossing
                                    last_crossing = None;
                                    if crossings % 2 == 0 {
                                        // outside
                                    } else {
                                        // inside
                                        all_interior_tiles.push(next_ivec);
                                    }
                                } else if last_crossing.is_some() {
                                    // if we land on an empty square
                                    // and the last crossing is *not* directly
                                    // before the empty space, then calculate if
                                    // we should cross
                                    let last_hash = next_ivec + IVec2::NEG_X;
                                    let last_hash_up = last_hash + IVec2::Y;
                                    let last_hash_down = last_hash + IVec2::NEG_Y;
                                    let last_hash_contains_up = travel.contains(&last_hash_up);
                                    let last_hash_contains_down = travel.contains(&last_hash_down);

                                    let last_cross = last_crossing.unwrap();
                                    let last_cross_up = last_cross + IVec2::Y;
                                    let last_cross_down = last_cross + IVec2::NEG_Y;
                                    let last_cross_contains_up = travel.contains(&last_cross_up);
                                    let last_cross_contains_down =
                                        travel.contains(&last_cross_down);
                                    if last_hash_contains_up && last_cross_contains_up
                                        || last_hash_contains_down && last_cross_contains_down
                                    {
                                        crossings += 1;
                                    }
                                    if crossings % 2 == 0 {
                                        // outside
                                    } else {
                                        // inside
                                        all_interior_tiles.push(next_ivec);
                                    }
                                    last_crossing = None;
                                } else {
                                    if crossings % 2 == 0 {
                                        // outside
                                    } else {
                                        // inside
                                        all_interior_tiles.push(next_ivec);
                                    }
                                }
                            }
                        }

                        (crossings, last_crossing, all_interior_tiles)
                    })
                },
            )
        })
        .flat_map(|x| x.2)
        .collect::<Vec<IVec2>>();

    let grid = travel
        .iter()
        .chain(interior_tiles.iter())
        .cloned()
        .collect::<Vec<IVec2>>();
    grid.len()
}

fn parse_grid(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, instructions) = separated_list1(line_ending, instruction)(input)?;

    Ok((input, instructions))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, direction) = alt((
        complete::char('R').map(|_| IVec2::X),
        complete::char('L').map(|_| IVec2::NEG_X),
        complete::char('U').map(|_| IVec2::Y),
        complete::char('D').map(|_| IVec2::NEG_Y),
    ))(input)?;

    let (input, amount) = delimited(space1, complete::i32, space1)(input)?;

    let (input, hex_code) = delimited(tag("(#"), hex_digit1, complete::char(')'))(input)?;

    Ok((
        input,
        Instruction {
            direction,
            amount,
            hex_code,
        },
    ))
}

#[allow(dead_code)]
fn print_grid(map: &[IVec2], y_bound: RangeInclusive<i32>, x_bound: RangeInclusive<i32>) {
    for y in y_bound.rev() {
        for x in x_bound.clone() {
            match map.contains(&IVec2::new(x, y)) {
                true => {
                    print!("#");
                }
                false => print!("."),
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./test.txt");
        let result = do_magic(input);
        assert_eq!(result, 62);
    }
}
