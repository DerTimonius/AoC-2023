use std::collections::HashMap;

use nom::{IResult, multi::separated_list1, character::complete::{line_ending, digit1, multispace0, multispace1}, sequence::{preceded, separated_pair, pair}, bytes::complete::tag, combinator::map_res};


#[derive(Debug)]
struct Card {
  // not needed in this puzzle, still nice to have
  id: usize,
  winning_numbers: Vec<u32>,
  betting_numbers: Vec<u32>,
}

impl Card {
  fn get_points(&self) -> Option<u32> {
    let points: HashMap<usize, u32> = HashMap::from([(1, 1), (2, 2), (3, 4), (4, 8), (5, 16), (6, 32), (7, 64), (8, 128), (9, 256), (10, 512)]);
    let in_both = self.betting_numbers.iter().filter(|&x| self.winning_numbers.contains(x)).count();
    points.get(&in_both).cloned()
  }
}

fn main() {
  let input = include_str!("./input.txt");
  let result = do_magic(input);
  println!("result: {}", result);
}

fn parse_u32(input: &str) -> IResult<&str, u32> {
  let (input, number) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;

  Ok((input, number))
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
  let (input, numbers) = separated_list1(multispace0, parse_u32)(input)?;
  Ok((input, numbers))
}

fn card(input: &str) -> IResult<&str, Card> {
  let (input, id) = preceded(pair(tag("Card"), multispace1), digit1)(input)?;

  let (input, (winning_numbers, betting_numbers)) = preceded(pair(tag(":"), multispace1),separated_pair(parse_numbers, pair(tag(" | "), multispace0), parse_numbers))(input)?;

  let id = id.parse::<usize>().expect("should be a number");
  Ok((input, Card {id, winning_numbers, betting_numbers}))
}

fn parse_game(input: &str) -> IResult<&str, Vec<Card>>  {
  let (input, cards) = separated_list1(line_ending, card)(input)?;

  Ok((input, cards))
}

fn do_magic(input: &str) -> u32 {
  let cards = parse_game(input).expect("should work");

  cards.1.iter().filter_map(|card| card.get_points()).sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
      let input = include_str!("./test.txt");
      let result = do_magic(input);
      assert_eq!(result, 13);
  }
}
