#[derive(Debug)]
struct Position(i32, i32);

fn main() {
  let input = include_str!("./input.txt");
  // let input = include_str!("./test.txt");
  let result = do_magic(input);
  println!("{}", result);
}

fn do_magic(input: &str) -> u32 {
  let row_numbers = input.lines().count();
  let col_numbers = input.lines().next().unwrap().chars().count();
  let mut numbers: Vec<(&str, Position)> = vec![];
  let mut stars: Vec<(char, Position)> = vec![];
  let number_regex = regex::Regex::new(r"\d+").unwrap();

  for (row, line) in input.lines().enumerate() {
      for matched in number_regex.find_iter(line) {
          numbers.push((
              matched.as_str(),
              Position(row as i32, matched.start() as i32),
          ));
      }
      for (col, char) in line.chars().enumerate() {
          if char == '*' {
              stars.push((char, Position(row as i32, col as i32)));
          }
      }
  }

  let mut gear_ratios: Vec<u32> = vec![];

  for (_, star_pos) in stars.iter() {
    let neighbors = [
            Position(star_pos.0 - 1, star_pos.1 - 1),
            Position(star_pos.0 - 1, star_pos.1),
            Position(star_pos.0 - 1, star_pos.1 + 1),
            Position(star_pos.0, star_pos.1 - 1),
            Position(star_pos.0, star_pos.1 + 1),
            Position(star_pos.0 + 1, star_pos.1 - 1),
            Position(star_pos.0 + 1, star_pos.1),
            Position(star_pos.0 + 1, star_pos.1 + 1),
        ]
        .into_iter()
        .filter(|p| {
            p.0 >= 0 && p.1 >= 0 && p.0 < row_numbers as i32 && p.1 < col_numbers as i32
        })
        .collect::<Vec<Position>>();
    let mut touched_numbers: Vec<u32> = Vec::new();
    for (number_str, number_start_pos) in numbers.iter() {
      let number_end_pos = Position(number_start_pos.0, number_start_pos.1 + number_str.len() as i32 - 1);
      for neighbor in neighbors.iter() {
                  if neighbor.0 == number_start_pos.0
                  && neighbor.1 >= number_start_pos.1
                  && neighbor.1 <= number_end_pos.1
                  {
                      touched_numbers.push(number_str.parse::<u32>().unwrap());
                      break;
                  }
              }
    }
    if touched_numbers.len() == 2 {
      gear_ratios.push(touched_numbers[0] * touched_numbers[1]);
    }
  }
  gear_ratios.iter().sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
      let input = include_str!("./test.txt");
      let result = do_magic(input);
      assert_eq!(result, 467835);
  }
}
