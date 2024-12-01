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
    let result = do_magic(input);
    println!("result: {}", result);
}

fn do_magic(input: &str) -> u32 {
    let rows = input.lines().count() as i32;
    let cols = input.lines().next().unwrap().len() as i32;
    let (_, grid) = parse_grid(Span::new(input)).expect("should not fail");

    let bounds = IVec2::new(rows, cols);
    let goal = IVec2::new(rows - 1, cols - 1);

    let output = dijkstra(
        &(IVec2::splat(0), VecDeque::from([IVec2::splat(0)])),
        |(position, queue)| {
            [
                IVec2::new(1, 0),
                IVec2::new(-1, 0),
                IVec2::new(0, -1),
                IVec2::new(0, 1),
            ]
            .into_iter()
            .filter_map(|pos_diff| {
                let next_position = pos_diff + *position;
                if (0..bounds.x).contains(&next_position.x)
                    && (0..bounds.y).contains(&next_position.y)
                {
                    if queue.len() > 2 && queue[1] == next_position {
                        return None;
                    }

                    let mut new_deque = queue.clone();
                    new_deque.push_front(next_position);
                    if new_deque.len() == 5 {
                        let dir = new_deque[1] - new_deque[0];
                        let a = new_deque[2] - new_deque[1];
                        let b = new_deque[3] - new_deque[2];
                        let c = new_deque[4] - new_deque[3];
                        // if we've moved in the same direction 4 times
                        let three_forward_check = [a, b, c].iter().all(|a_dir| a_dir == &dir);

                        if three_forward_check {
                            None
                        } else {
                            new_deque.pop_back();
                            Some((next_position, new_deque))
                        }
                    } else {
                        Some((next_position, new_deque))
                    }
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
        |(win, _deque)| win == &goal,
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
    fn part1_works() {
        let input = include_str!("./test.txt");
        let result = do_magic(input);
        assert_eq!(result, 102);
    }
}
