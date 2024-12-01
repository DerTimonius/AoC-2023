use nom::{sequence::{separated_pair, preceded, tuple}, character::complete::{line_ending, multispace1, multispace0, digit1}, bytes::complete::{take_until1, tag}, IResult, multi::separated_list1};

#[derive(Debug)]
struct Race {
  time: u32,
  distance: u32,
}

impl Race {
  fn get_winning_ways(&self) -> u32 {
    let mut sum: u32 = 0;

    for i in 0..self.time {
      if (self.time - i) * i > self.distance {
        sum += 1
      }
    }
    sum
  }
}

fn main() {
  // let input = include_str!("./input.txt");
  let input = include_str!("./test.txt");
  let result = do_magic(input);
  println!("{}", result);
}

fn do_magic(input: &str) -> u32 {
  let (_, races) = parse_races(input).expect("should parse");
  println!("races: {:?}", races);

  let ways: Vec<u32> = races.iter().map(|race| race.get_winning_ways()).collect();

  ways.iter().product()
}

fn parse_races(input: &str) -> IResult<&str, Vec<Race>> {
  let (input, (times, distances)) = separated_pair(preceded(tuple((take_until1(":"), tag(":"), multispace1)), separated_list1(multispace0, digit1)), line_ending, preceded(tuple((take_until1(":"), tag(":"), multispace1)), separated_list1(multispace0, digit1)))(input)?;

  let races: Vec<Race> = times.iter().zip(distances.iter()).map(|(&time, &distance)| Race {distance: distance.parse::<u32>().unwrap_or(0), time: time.parse::<u32>().unwrap_or(0)}).collect();

  Ok((input, races))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part1_works() {
      let input = include_str!("./test.txt");
      let result = do_magic(input);
      assert_eq!(result, 288);
  }
}
