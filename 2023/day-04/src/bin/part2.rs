use nom::{IResult, multi::separated_list1, character::complete::{line_ending, digit1, multispace0, multispace1}, sequence::{preceded, separated_pair, pair}, bytes::complete::tag, combinator::map_res};


#[derive(Debug)]
struct Card {
  id: usize,
  winning_numbers: Vec<u32>,
  betting_numbers: Vec<u32>,
  amount: usize,
}

impl Card {
  fn get_points(&self) -> usize {
    let in_both = self.betting_numbers.iter().filter(|&x| self.winning_numbers.contains(x)).count();
    in_both
  }

}

fn main() {
  // let input = include_str!("./test.txt");
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
  Ok((input, Card {id, winning_numbers, betting_numbers, amount: 1}))
}

fn parse_game(input: &str) -> IResult<&str, Vec<Card>>  {
  let (input, cards) = separated_list1(line_ending, card)(input)?;

  Ok((input, cards))
}

fn do_magic(input: &str) -> usize {
  let parsed = parse_game(input).expect("should work");
  let mut cards = parsed.1;

  for i in 0..cards.len() {
    let (current_cards, future_cards) = cards.split_at_mut(i + 1);

    let current_card = &current_cards[i];
    let points = current_card.get_points();
    let mut copies = current_card.amount;

    while copies > 0 {
      for j in 1..=points {
        if let Some(next_card) = future_cards.iter_mut().find(|c| c.id == (current_card.id + j)) {
          next_card.amount += 1;
        }
      }
      copies -= 1
    }
  }

  cards.iter().map(|card| card.amount).sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
      let input = include_str!("./test.txt");
      let result = do_magic(input);
      assert_eq!(result, 30);
  }
}
