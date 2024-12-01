use std::ops::Range;

use nom::{
  bytes::complete::take_until,
  character::complete::{self, line_ending, space1},
  multi::{many1, separated_list1},
  sequence::tuple,
  IResult, Parser,
};
use nom_supreme::{tag::complete::tag, ParserExt};

#[derive(Debug)]
struct Plan {
  maps: Vec<(Range<u64>, Range<u64>)>,
}

impl Plan {
  fn get_next(&self, source: u64) -> u64 {
    let valid_map = self.maps.iter().find(|(source_range, _)| source_range.contains(&source));
    let Some((source_range, destination_range)) = valid_map else { return source };

    let offset = source - source_range.start;

    destination_range.start + offset
  }
}

fn main() {
  let input = include_str!("./input.txt");
  // let input = include_str!("./test.txt");
  let result = do_magic(input);
  println!("{}", result);
}

fn do_magic(input: &str) -> u64 {
  let (_, (seeds, plans)) = parse_plan(input).expect("hopefully parses");

  let locations = seeds.iter().map(|seed| {
    plans.iter().fold(*seed, |seed, plan| {
      plan.get_next(seed)
    })
  }).collect::<Vec<u64>>();

  *locations.iter().min().expect("there should be at least one item")
}

fn line(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
  let (input, (destination, source, num)) = tuple((complete::u64, complete::u64.preceded_by(tag(" ")), complete::u64.preceded_by(tag(" "))))(input)?;

  Ok((input, ( source..(source + num), destination..(destination + num))))
}

fn parse_map(input: &str) -> IResult<&str, Plan> {
  let (input, map) = take_until("map:")
  .precedes(tag("map:"))
  .precedes(
      many1(line_ending.precedes(line))
          .map(|maps| Plan { maps }),
  )
  .parse(input)?;

  Ok((input, map))
}

fn parse_plan(input: &str) -> IResult<&str, (Vec<u64>, Vec<Plan>)> {
  let (input, seeds) = tag("seeds: ")
  .precedes(separated_list1(space1, complete::u64))
  .parse(input)?;
let (input, maps) = many1(parse_map)(input)?;

Ok((input, (seeds, maps)))
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
      let input = include_str!("./test.txt");
      let result = do_magic(input);
      assert_eq!(result, 35);
  }
}
