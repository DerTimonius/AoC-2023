use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coordinates {
    x: u32,
    y: u32,
    z: u32,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Brick {
    start: Coordinates,
    end: Coordinates,
}

impl Brick {
    fn direction(&self) -> (char, u32) {
        if self.start.x != self.end.x {
            ('x', self.end.x - self.start.x)
        } else if self.start.y != self.end.y {
            ('y', self.end.y - self.start.y)
        } else {
            ('z', self.end.z - self.start.z)
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");
    // let input = include_str!("./test.txt");
    let result = do_magic(input);
    println!("result: {}", result);
}

fn do_magic(input: &str) -> u32 {
    let (_, bricks) = parse_input(input).expect("should parse without failing");
    let mut bricks = bricks.clone();
    bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z));
    drop_bricks(&mut bricks);
    let mut sum = 0;
    let (supports_map, supported_by_map) = find_support_relationships(&bricks);

    for brick in &bricks {
        if let Some(map) = supports_map.get(brick) {
            match map.len() {
                0 => sum += 1,
                _ => {
                    if map
                        .iter()
                        .map(|supp| {
                            if let Some(x) = supported_by_map.get(supp) {
                                x.len()
                            } else {
                                0
                            }
                        })
                        .all(|x| x > 1)
                    {
                        sum += 1
                    }
                }
            }
        } else {
            panic!("should never happen")
        }
    }
    sum
}

fn find_support_relationships(
    bricks: &Vec<Brick>,
) -> (HashMap<&Brick, Vec<&Brick>>, HashMap<&Brick, Vec<&Brick>>) {
    let mut supports_map: HashMap<&Brick, Vec<&Brick>> = HashMap::new();
    let mut supported_by_map: HashMap<&Brick, Vec<&Brick>> = HashMap::new();

    for brick in bricks {
        let supports = bricks
            .iter()
            .filter(|&other_brick| {
                other_brick.start.z == brick.end.z + 1
                    && match brick.direction() {
                        ('x', _) => {
                            (other_brick.start.x..=other_brick.end.x).contains(&brick.start.x)
                                && (other_brick.start.y..=other_brick.end.y)
                                    .contains(&brick.start.y)
                        }
                        ('y', _) => {
                            (other_brick.start.y..=other_brick.end.y).contains(&brick.start.y)
                                && (other_brick.start.x..=other_brick.end.x)
                                    .contains(&brick.start.x)
                        }
                        ('z', _) => {
                            (other_brick.start.x..=other_brick.end.x).contains(&brick.start.x)
                                && (other_brick.start.y..=other_brick.end.y)
                                    .contains(&brick.start.y)
                        }
                        _ => false,
                    }
            })
            .collect::<Vec<&Brick>>();

        let supported_by = bricks
            .iter()
            .filter(|&other_brick| {
                brick.start.z == other_brick.end.z - 1
                    && match brick.direction() {
                        ('x', _) => {
                            (brick.start.x..=brick.end.x).contains(&other_brick.start.x)
                                && (other_brick.start.y..=other_brick.end.y)
                                    .contains(&brick.start.y)
                        }
                        ('y', _) => {
                            (brick.start.y..=brick.end.y).contains(&other_brick.start.y)
                                && (other_brick.start.x..=other_brick.end.x)
                                    .contains(&brick.start.x)
                        }
                        ('z', _) => {
                            (other_brick.start.x..=other_brick.end.x).contains(&brick.start.x)
                                && (other_brick.start.y..=other_brick.end.y)
                                    .contains(&brick.start.y)
                        }
                        _ => false,
                    }
            })
            .collect::<Vec<&Brick>>();

        supports_map.insert(brick, supports);
        supported_by_map.insert(brick, supported_by);
    }
    (supports_map, supported_by_map)
}

fn drop_bricks(bricks: &mut Vec<Brick>) {
    let mut grid = create_grid(bricks);
    for brick in bricks.iter_mut() {
        let (dir, num) = brick.direction();
        match dir {
            'x' => {
                let highest_point = (brick.start.x..=brick.end.x)
                    .map(|x| grid[x as usize][brick.start.y as usize])
                    .max()
                    .unwrap_or(0);

                brick.start.z = highest_point + 1;
                brick.end.z = highest_point + 1;
                for x in brick.start.x..=brick.end.x {
                    grid[x as usize][brick.end.y as usize] = highest_point + 1
                }
            }
            'y' => {
                let highest_point = (brick.start.y..=brick.end.y)
                    .map(|y| grid[brick.start.x as usize][y as usize])
                    .max()
                    .unwrap_or(0);

                brick.start.z = highest_point + 1;
                brick.end.z = highest_point + 1;
                for y in brick.start.y..=brick.end.y {
                    grid[brick.start.x as usize][y as usize] = highest_point + 1
                }
            }
            'z' => {
                let highest_point = grid[brick.start.x as usize][brick.start.y as usize].max(1);
                let diff = highest_point.abs_diff(num);
                brick.start.z -= diff;
                brick.end.z -= diff;

                grid[brick.start.x as usize][brick.start.y as usize] = brick.end.z
            }
            _ => panic!("Encountered a really weird direction"),
        }
    }
}

fn create_grid(bricks: &Vec<Brick>) -> Vec<Vec<u32>> {
    let rows = bricks
        .iter()
        .map(|brick| brick.start.x.max(brick.end.x))
        .max()
        .unwrap_or(0)
        + 1;
    let cols = bricks
        .iter()
        .map(|brick| brick.start.y.max(brick.end.y))
        .max()
        .unwrap_or(0)
        + 1;

    let grid = vec![vec![0; cols as usize]; rows as usize];

    grid
}

fn parse_input(input: &str) -> IResult<&str, Vec<Brick>> {
    let (input, bricks) = separated_list1(line_ending, brick)(input)?;

    Ok((input, bricks))
}

fn brick(input: &str) -> IResult<&str, Brick> {
    let (input, (start, end)) = separated_pair(
        separated_list1(tag(","), complete::u32),
        tag("~"),
        separated_list1(tag(","), complete::u32),
    )(input)?;

    let start = Coordinates {
        x: start[0],
        y: start[1],
        z: start[2],
    };
    let end = Coordinates {
        x: end[0],
        y: end[1],
        z: end[2],
    };

    Ok((input, Brick { start, end }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./test.txt");
        let result = do_magic(input);
        assert_eq!(result, 5);
    }
}
