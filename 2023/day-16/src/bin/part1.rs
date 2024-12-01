use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Direction {
    row: isize,
    col: isize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Node {
    x: isize,
    y: isize,
}

fn main() {
    let input = include_str!("./input.txt");
    // let input = include_str!("./test.txt");
    let result = do_magic(input);
    println!("result: {}", result);
}

fn do_magic(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    traverse_grid(&mut grid)
}

fn traverse_grid(grid: &mut Vec<Vec<char>>) -> usize {
    let mut visited = HashMap::new();
    let mut directions_queue: VecDeque<(Node, Direction)> =
        VecDeque::from([(Node { x: 0, y: -1 }, Direction { row: 0, col: 1 })]);
    let rows = grid.len();
    let cols = grid[0].len();

    while let Some((current_node, direction)) = directions_queue.pop_front() {
        let new_coordinates = Direction {
            row: current_node.x as isize + direction.row,
            col: current_node.y as isize + direction.col,
        };

        if new_coordinates.row < 0
            || new_coordinates.row >= rows as isize
            || new_coordinates.col < 0
            || new_coordinates.col >= cols as isize
        {
            println!("out of bounds");
            continue;
        }

        let new_node = Node {
            x: new_coordinates.row,
            y: new_coordinates.col,
        };
        let new_char = grid[new_node.x as usize][new_node.y as usize];

        visited.entry(new_node).or_insert(vec![]);

        let new_direction;
        if new_char == '.'
            || new_char == '-' && direction.col == 1
            || new_char == '-' && direction.col == -1
            || new_char == '|' && direction.row == 1
            || new_char == '|' && direction.row == -1
        {
            new_direction = direction;
        } else if new_char == '/' {
            if direction.row == 1 {
                new_direction = Direction { row: 0, col: -1 };
            } else if direction.row == -1 {
                new_direction = Direction { row: 0, col: 1 };
            } else if direction.col == 1 {
                new_direction = Direction { row: -1, col: 0 };
            } else {
                new_direction = Direction { row: 1, col: 0 };
            }
        } else if new_char == '\\' {
            if direction.row == 1 {
                new_direction = Direction { row: 0, col: 1 };
            } else if direction.row == -1 {
                new_direction = Direction { row: 0, col: -1 };
            } else if direction.col == 1 {
                new_direction = Direction { row: 1, col: 0 };
            } else {
                new_direction = Direction { row: -1, col: 0 };
            }
        } else if new_char == '-' {
            directions_queue.push_back((new_node, Direction { row: 0, col: -1 }));
            new_direction = Direction { row: 0, col: 1 };
        } else {
            directions_queue.push_back((new_node, Direction { row: -1, col: 0 }));
            new_direction = Direction { row: 1, col: 0 };
        }

        if let Some(known_directions) = visited.get_mut(&new_node) {
            if known_directions.contains(&new_direction) {
                println!("loop detected");
                continue;
            } else {
                known_directions.push(new_direction);
            }
        } else {
            panic!("should never happen!")
        }

        directions_queue.push_back((new_node, new_direction));
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./test.txt");
        let result = do_magic(input);
        assert_eq!(result, 46);
    }
}
