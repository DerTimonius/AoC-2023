use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::{self, alpha1, line_ending, newline},
  combinator::map,
  multi::separated_list1,
  sequence::{delimited, tuple},
  IResult,
};

#[derive(Debug, PartialEq, Eq)]
enum Condition<'a> {
  Field(&'a str, &'a str, u32),
  True,
}

#[derive(Debug, PartialEq, Eq)]
struct Rule<'a> {
  condition: Condition<'a>,
  outcome: &'a str,
}

#[derive(Debug, PartialEq, Eq)]
enum ParsedRule<'a> {
  Simple(&'a str),
  Complex(Rule<'a>),
}

#[derive(Debug, Eq, PartialEq)]
struct Workflow<'a> {
  name: &'a str,
  rules: Vec<ParsedRule<'a>>,
}

#[derive(Debug)]
struct Part {
  x: u32,
  m: u32,
  a: u32,
  s: u32,
}

fn main() {
  let input = include_str!("./input.txt");
  // let input = include_str!("./test.txt");
  let result = do_magic(input);
  println!("result: {}", result);
}

fn do_magic(input: &str) -> u32 {
  let (_, (workflows, parts)) = parse_input(input).expect("should parse without failing");

  let accepted_sum = parts.iter().map(|part| {
      let mut workflow_name = String::from("in");
      while workflow_name != "A" && workflow_name != "R" {
          let workflow = workflows.iter().find(|workflow| workflow.name == workflow_name.as_str()).expect("should exist");
          workflow_name = process_workflow(workflow, part)
      }

      match workflow_name.as_str() {
          "A" => part.x + part.m + part.a + part.s,
          _ => 0
      }
  }).sum::<u32>();
  accepted_sum
}

fn process_workflow(workflow: &Workflow, part: &Part) -> String {
  let mut current_rule_index = 0;

  while current_rule_index < workflow.rules.len() {
      if let Some(result) = process_rule(&workflow.rules[current_rule_index], part) {
          return result;
      }
      current_rule_index += 1
  }

  unreachable!("some workflow has to be found")
}

fn process_rule(rule: &ParsedRule, part: &Part) -> Option<String> {
  match rule {
      ParsedRule::Simple(outcome) => Some(outcome.to_string()),
      ParsedRule::Complex(complex_rule) => {
          match &complex_rule.condition {
              Condition::Field(field, op, value) => {
                  let field_value = match *field {
                      "x" => part.x,
                      "m" => part.m,
                      "a" => part.a,
                      "s" => part.s,
                      _ => panic!("found a rule that should not exist"),
                  };

                  // Check the condition
                  let condition_result = match *op {
                      "<" => field_value < *value,
                      ">" => field_value > *value,
                      _ => panic!("found a condition that should not exist"),
                  };

                  if condition_result {
                      Some(complex_rule.outcome.to_string())
                  } else {
                      None // Condition not met
                  }
              }
              Condition::True => Some(complex_rule.outcome.to_string()),
          }
      }
  }
}


fn parse_input(input: &str) -> IResult<&str, (Vec<Workflow>, Vec<Part>)> {
  let (input, (workflows, _, _, parts)) =
      tuple((parse_workflows, line_ending, newline, parse_parts))(input)?;

  Ok((input, (workflows, parts)))
}

fn parse_workflows(input: &str) -> IResult<&str, Vec<Workflow>> {
  let (input, workflows) = separated_list1(line_ending, parse_workflow)(input)?;

  Ok((input, workflows))
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
  let (input, name) = alpha1(input)?;
  let (input, rules) = parse_rules(input)?;

  Ok((input, Workflow { name, rules }))
}

fn parse_rules(input: &str) -> IResult<&str, Vec<ParsedRule>> {
  let (input, rules) =
      delimited(tag("{"), separated_list1(tag(","), parse_rule), tag("}"))(input)?;

  Ok((input, rules))
}

fn parse_condition(input: &str) -> IResult<&str, Condition> {
  let (input, field) = alpha1(input)?;
  let (input, op) = alt((tag("<="), tag(">="), tag("<"), tag(">"), tag("="), tag("")))(input)?;
  let (input, value) = complete::u32(input)?;

  Ok((input, Condition::Field(field, op, value)))
}

fn parse_outcome(input: &str) -> IResult<&str, &str> {
  alpha1(input)
}

fn parse_rule(input: &str) -> IResult<&str, ParsedRule> {
  alt((
      map(
          tuple((parse_condition, tag(":"), parse_outcome)),
          |(condition, _, outcome)| ParsedRule::Complex(Rule { condition, outcome }),
      ),
      map(alpha1, ParsedRule::Simple),
  ))(input)
}

fn parse_parts(input: &str) -> IResult<&str, Vec<Part>> {
  let (input, parts) =
      separated_list1(line_ending, delimited(tag("{"), parse_part, tag("}")))(input)?;

  Ok((input, parts))
}

fn parse_part(input: &str) -> IResult<&str, Part> {
  let (input, categories) =
      separated_list1(tag(","), tuple((alpha1, tag("="), complete::u32)))(input)?;

  let part = categories.iter().fold(
      Part {
          x: 0,
          m: 0,
          a: 0,
          s: 0,
      },
      |mut acc, (name, _, number)| {
          match *name {
              "x" => acc.x = *number,
              "m" => acc.m = *number,
              "a" => acc.a = *number,
              "s" => acc.s = *number,
              _ => panic!("unexpected character"),
          }
          acc
      },
  );

  Ok((input, part))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
      let input = include_str!("./test.txt");
      let result = do_magic(input);
      assert_eq!(result, 19114);
  }
}
