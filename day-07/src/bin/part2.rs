use std::{str::Chars, ops::Deref};

use itertools::{Itertools, Position};

use nom::{multi::separated_list1, character::complete::{line_ending, multispace1, digit1}, IResult, sequence::separated_pair, bytes::complete::take_until};

#[derive(Debug, Copy, Clone)]
enum HandType {
  FiveOfAKind = 6,
  FourOfAKind = 5,
  FullHouse = 4,
  ThreeOfAKind = 3,
  TwoPair = 2,
  OnePair = 1,
  HighCard = 0,
}

#[derive(Debug)]
struct Hand<'a> {
  cards: Chars<'a>,
  bid: u32,
}

impl<'a> Hand<'a> {
  fn score(&self) -> (HandType, (u32, u32, u32, u32, u32)) {
    use HandType::*;

    let counts = self.cards.clone().counts();
    let values = if let Some(jokers) = counts.get(&'J') {
      if *jokers == 5 {
        "5".to_string()
      } else {
        let result = counts.iter().filter_map(|(k, v)| (k != &'J').then_some(v)).sorted().with_position()
        .map(|(position, value)| match position {
            Position::Last | Position::Only => {
                value + jokers
            }
            _ => *value,
        })
        .join("");
      result
      }
    } else  {counts.values().sorted().join("")};
    let hand_type = match values.deref() {
        "5" => FiveOfAKind,
        "14" => FourOfAKind,
        "23" => FullHouse,
        "113" => ThreeOfAKind,
        "122" => TwoPair,
        "1112" => OnePair,
        "11111" => HighCard,
        value => panic!(
            "should never happen. Encountered `{}`",
            value
        ),
    };
    let card_scores = self.cards.clone()
        .map(|card| match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 0,
            'T' => 10,
            value => value.to_digit(10).unwrap(),
        })
        .collect_tuple()
        .unwrap();
    (hand_type, card_scores)
  }
}

fn main() {
  let input = include_str!("./input.txt");
  // let input = include_str!("./test.txt");
  let result = do_magic(input);
  println!("{}", result);
}

fn do_magic(input: &str) -> u32 {
  let (_, hands) = parse_hands(input).expect("should parse without errors");
  hands
    .iter()
    .map(|card| (card, card.bid, card.score()))
    .sorted_by_key(|x| (x.2 .0 as u8, x.2 .1))
    .enumerate()
    .map(|(index, (_hand, bid, _))| {
      (index as u32 + 1) * bid
    })
    .sum::<u32>()
}

fn parse_hands(input: &str) -> IResult<&str, Vec<Hand>> {
  let (input, hands) = separated_list1(line_ending, hand)(input)?;

  Ok((input, hands))
}

fn hand(input: &str) -> IResult<&str, Hand> {
  let (input, (cards, bid)) = separated_pair(take_until(" "), multispace1, digit1)(input)?;

  let bid = bid.parse::<u32>().unwrap_or(0);
  let cards = cards.chars();

  Ok((input, Hand {cards, bid}))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn part_two_works() {
      let input = include_str!("./test.txt");
      let result = do_magic(input);
      assert_eq!(result, 5905);
  }
}
