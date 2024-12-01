#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Rock {
    Round,
    Cube,
    Empty,
}

fn main() {
    // let input = include_str!("./input.txt");
    let input = include_str!("./test.txt");
    let result = do_magic(input);
    println!("result: {}", result);
}

fn do_magic(input: &str) -> usize {
    let mut grid: Vec<Vec<Rock>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Rock::Round,
                    '#' => Rock::Cube,
                    '.' => Rock::Empty,
                    _ => panic!("Invalid rock type"),
                })
                .collect()
        })
        .collect();

  let mut seen_states: Vec<Vec<Vec<Rock>>> = vec![grid.clone()];

  loop {
      grid = cycle(&grid);
      if let Some(index) = seen_states.iter().position(|x| x == &grid) {
          let cycle_length = seen_states.len() - index;
          let cycle_start = index;
          let final_grid =
              seen_states[cycle_start + (1000000000 - cycle_start) % cycle_length].clone();

          return calculate_load(&final_grid);
      }
      seen_states.push(grid.clone());
  }
}

fn calculate_load(grid: &Vec<Vec<Rock>>) -> usize {
  let mut load = 0;
  for (vertical_pos, line) in grid.iter().enumerate() {
    for (_, rock) in line.iter().enumerate() {
        if *rock == Rock::Round {
            load += grid.len() - vertical_pos;
        }
    }
}

load
}

fn tilt_north(grid: &Vec<Vec<Rock>>) -> Vec<Vec<Rock>> {
    let mut empty_above: Vec<usize> = vec![0; grid[0].len()];
    let mut new_grid = grid.clone();

    for (vertical_pos, line) in grid.iter().enumerate() {
        for (horizontal_pos, rock) in line.iter().enumerate() {
            match rock {
                Rock::Empty => {
                    empty_above[horizontal_pos] += 1;
                }
                Rock::Cube => {
                    empty_above[horizontal_pos] = 0;
                }
                Rock::Round => {
                    let new_rock_pos = vertical_pos - empty_above[horizontal_pos];
                    new_grid[vertical_pos][horizontal_pos] = Rock::Empty;
                    new_grid[new_rock_pos][horizontal_pos] = Rock::Round;
                }
            }
        }
    }

    new_grid
}
fn tilt_south(grid: &Vec<Vec<Rock>>) -> Vec<Vec<Rock>> {
  let mut empty_above: Vec<usize> = vec![0; grid[0].len()];
    let mut new_grid = grid.clone();
    let grid_height = grid.len();

    for (vertical_pos, line) in grid.iter().rev().enumerate() {
        for (horizontal_pos, rock) in line.iter().enumerate() {
            match rock {
                Rock::Empty => {
                    empty_above[horizontal_pos] += 1;
                }
                Rock::Cube => {
                    empty_above[horizontal_pos] = 0;
                }
                Rock::Round => {
                    let new_rock_pos = vertical_pos - empty_above[horizontal_pos];
                    new_grid[grid_height - 1 - vertical_pos][horizontal_pos] = Rock::Empty;
                    new_grid[grid_height - 1 - new_rock_pos][horizontal_pos] = Rock::Round;
                }
            }
        }
    }

    new_grid
}

fn tilt_west(grid: &Vec<Vec<Rock>>) -> Vec<Vec<Rock>> {
    let mut empty_left: Vec<usize> = vec![0; grid.len()];
    let mut new_grid = grid.clone();

    for horizontal_pos in 0..grid[0].len() {
        for vertical_pos in 0..grid.len() {
            let rock = grid[vertical_pos][horizontal_pos];
            match rock {
                Rock::Empty => {
                    empty_left[vertical_pos] += 1;
                }
                Rock::Cube => {
                    empty_left[vertical_pos] = 0;
                }
                Rock::Round => {
                    let new_rock_pos = horizontal_pos - empty_left[vertical_pos];
                    new_grid[vertical_pos][horizontal_pos] = Rock::Empty;
                    new_grid[vertical_pos][new_rock_pos] = Rock::Round;
                }
            }
        }
    }

    new_grid
}

fn tilt_east(grid: &Vec<Vec<Rock>>) -> Vec<Vec<Rock>> {
  let mut empty_left: Vec<usize> = vec![0; grid.len()];
    let mut new_grid = grid.clone();

    for horizontal_pos in (0..grid[0].len()).rev() {
        for vertical_pos in 0..grid.len() {
            let rock = grid[vertical_pos][horizontal_pos];
            match rock {
                Rock::Empty => {
                    empty_left[vertical_pos] += 1;
                }
                Rock::Cube => {
                    empty_left[vertical_pos] = 0;
                }
                Rock::Round => {
                    let new_rock_pos = horizontal_pos + empty_left[vertical_pos];
                    new_grid[vertical_pos][horizontal_pos] = Rock::Empty;
                    new_grid[vertical_pos][new_rock_pos] = Rock::Round;
                }
            }
        }
    }

    new_grid
}

fn cycle(grid: &Vec<Vec<Rock>>) -> Vec<Vec<Rock>> {
  let new_grid = tilt_north(&grid);
  let new_grid = tilt_west(&new_grid);
  let new_grid = tilt_south(&new_grid);
  tilt_east(&new_grid)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./test.txt");
        let result = do_magic(input);
        assert_eq!(result, 64);
    }
}
