
use nom::{sequence::{separated_pair, preceded, tuple}, character::complete::{line_ending, multispace1, multispace0, digit1}, bytes::complete::{take_until1,  tag}, IResult, multi::separated_list1};

#[derive(Debug)]
struct Race {
  time: u128,
  distance: u128,
}

impl Race {
  fn get_winning_ways(&self) -> u128 {
    let mut sum: u128 = 0;

    for i in 1..self.time {
      if (self.time - i) * i > self.distance {
        sum += 1
      }
    }
    sum
  }
}

fn main() {
  let input = include_str!("./input.txt");
  // let input = include_str!("./test.txt");
  let result = do_magic(input);
  println!("{}", result);
}

fn do_magic(input: &str) -> u128 {
  let (_, races) = parse_races(input).expect("should parse");
  races.get_winning_ways()

}

fn parse_races(input: &str) -> IResult<&str, Race> {
  let (input, (times, distances)) = separated_pair(preceded(tuple((take_until1(":"), tag(":"), multispace1)), separated_list1(multispace0, digit1)), line_ending, preceded(tuple((take_until1(":"), tag(":"), multispace1)), separated_list1(multispace0, digit1)))(input)?;

  let time = times.concat().parse::<u128>().unwrap_or(0);
  let distance = distances.concat().parse::<u128>().unwrap_or(0);


  Ok((input, Race {time, distance}))
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part2_works() {
      let input = include_str!("./test.txt");
      let result = do_magic(input);
      assert_eq!(result, 71503);
  }
}
