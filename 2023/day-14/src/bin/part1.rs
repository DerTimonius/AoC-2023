#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Rock {
    Round,
    Cube,
    Empty,
}

fn main() {
    let input = include_str!("./input.txt");
    // let input = include_str!("./test.txt");
    let result = do_magic(input);
    println!("result: {}", result);
}

fn do_magic(input: &str) -> usize {
    let lines: Vec<Vec<Rock>> = input
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

    let mut empty_above: Vec<usize> = vec![0; lines[0].len()];
    let mut result = 0;

    for (vertical_pos, line) in lines.iter().enumerate() {
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
                    result += lines.len() - new_rock_pos;
                }
            }
        }
    }

    result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./test.txt");
        let result = do_magic(input);
        assert_eq!(result, 136);
    }
}
