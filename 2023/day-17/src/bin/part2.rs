use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

use glam::IVec2;
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::{position, LocatedSpan};
use pathfinding::directed::dijkstra::dijkstra;

type Span<'a> = LocatedSpan<&'a str>;

fn main() {
    let input = include_str!("./input.txt");
    // let input = include_str!("./test.txt");
    let min = 4;
    let max = 10;
    let result = do_magic(input, min, max);
    println!("result: {}", result);
}

fn do_magic(input: &str, min: usize, max: usize) -> u32 {
    let rows = input.lines().count() as i32;
    let cols = input.lines().next().unwrap().len() as i32;
    let (_, grid) = parse_grid(Span::new(input)).expect("should not fail");

    let bounds = IVec2::new(rows, cols);
    let goal = IVec2::new(rows - 1, cols - 1);

    let output = dijkstra(
        &(IVec2::splat(0), VecDeque::from([IVec2::splat(0)])),
        |(position, queue)| {
            let diffs: Vec<IVec2> = queue.iter().tuple_windows().map(|(a, b)| *a - *b).collect();
            let last_diff = diffs.get(0);

            let maybe_first_diff_count = diffs.iter().dedup_with_count().next();
            let options = if let Some(diff_count) = maybe_first_diff_count {
                let num_consecutive_straight_diffs = diff_count.0;
                let must_turn = num_consecutive_straight_diffs == max;
                let must_go_straight = num_consecutive_straight_diffs < min;

                if must_turn {
                    [
                        IVec2::new(1, 0),
                        IVec2::new(-1, 0),
                        IVec2::new(0, -1),
                        IVec2::new(0, 1),
                    ]
                    .into_iter()
                    .filter(|option| option != last_diff.unwrap())
                    .collect::<Vec<IVec2>>()
                } else if must_go_straight {
                    vec![*last_diff.unwrap()]
                } else {
                    vec![
                        IVec2::new(1, 0),
                        IVec2::new(-1, 0),
                        IVec2::new(0, -1),
                        IVec2::new(0, 1),
                    ]
                }
            } else {
                vec![
                    IVec2::new(1, 0),
                    IVec2::new(-1, 0),
                    IVec2::new(0, -1),
                    IVec2::new(0, 1),
                ]
            };
            options
                .into_iter()
                .filter_map(|pos_diff| {
                    let next_position = pos_diff + *position;
                    if (0..bounds.x).contains(&next_position.x)
                        && (0..bounds.y).contains(&next_position.y)
                    {
                        if queue.len() > 2 && queue[1] == next_position {
                            return None;
                        }

                        let mut new_queue = queue.clone();
                        new_queue.push_front(next_position);

                        if new_queue.len() > 14 {
                            new_queue.pop_back();
                        }
                        Some((next_position, new_queue))
                    } else {
                        None
                    }
                })
                .map(|pos| {
                    let next_cost = *grid.get(&pos.0).unwrap();
                    (pos, next_cost)
                })
                .collect::<Vec<((IVec2, VecDeque<IVec2>), u32)>>()
        },
        |(win, queue)| {
            let diffs: Vec<IVec2> = queue.iter().tuple_windows().map(|(a, b)| *a - *b).collect();

            let maybe_first_diff_count = diffs.iter().dedup_with_count().next();

            maybe_first_diff_count.is_some_and(|(count, _)| count >= min) && win == &goal
        },
    );
    output.unwrap().1
}

fn parse_grid(input: Span) -> IResult<Span, HashMap<IVec2, u32>> {
    let (input, grid) = separated_list1(line_ending, many1(get_num))(input)?;
    let map = grid
        .into_iter()
        .flat_map(|v| v.into_iter())
        .collect::<HashMap<IVec2, u32>>();

    Ok((input, map))
}

fn get_num(input: Span) -> IResult<Span, (IVec2, u32)> {
    let (input, pos) = position(input)?;
    let (input, num) = one_of("0123456789")(input)?;

    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;

    Ok((input, (IVec2::new(x, y), num.to_digit(10).unwrap())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works() {
        let input = include_str!("./test.txt");
        let result = do_magic(input, 4, 10);
        assert_eq!(result, 94);
    }
}
