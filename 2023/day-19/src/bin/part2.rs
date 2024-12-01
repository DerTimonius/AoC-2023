use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq, Clone)]
struct Condition<'a> {
    part: &'a str,
    operator: &'a str,
    number: u64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Rule<'a> {
    condition: Condition<'a>,
    outcome: &'a str,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum ParsedRule<'a> {
    Simple(&'a str),
    Complex(Rule<'a>),
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<ParsedRule<'a>>,
}

fn main() {
    let input = include_str!("./input.txt");
    // let input = include_str!("./test.txt");
    let result = do_magic(input);
    println!("result: {}", result);
}

fn do_magic(input: &str) -> u64 {
    let (input, _) = input.split_once("\n\n").expect("should find this");
    let (_, workflows) = parse_workflows(input).expect("should parse without failing");

    let mut accepted_part_combination: u64 = 0;
    let mut accepted_paths = vec![];
    let mut queue = vec![(ParsedRule::Simple("in"), vec![])];
    while let Some((next, constraints)) = queue.pop() {
        match next {
            ParsedRule::Simple("A") => {
                accepted_paths.push(constraints);
                continue;
            }
            ParsedRule::Simple("R") => {
                continue;
            }
            ParsedRule::Simple(n) => {
                let mut inverted_constraints = constraints.clone();
                for rule in workflows.get(n).unwrap() {
                    match rule {
                        ParsedRule::Simple(_) => {
                            queue.push((rule.clone(), inverted_constraints.clone()));
                        }
                        ParsedRule::Complex(oper) => {
                            queue.push((rule.clone(), inverted_constraints.clone()));
                            let mut oper = oper.clone();
                            oper.condition = if oper.condition.operator == "<" {
                                Condition {
                                    number: oper.condition.number - 1,
                                    operator: ">",
                                    ..oper.condition
                                }
                            } else {
                                Condition {
                                    number: oper.condition.number + 1,
                                    operator: "<",
                                    ..oper.condition
                                }
                            };
                            inverted_constraints.push(oper.clone());
                        }
                    }
                }
            }
            ParsedRule::Complex(rule) => {
                let mut constraints = constraints.clone();
                // // add the new operation to constraints
                constraints.push(rule.clone());
                queue.push((ParsedRule::Simple(rule.outcome), constraints.clone()));
            }
        }
    }

    for path in accepted_paths {
        let mut min_x: u64 = 1;
        let mut max_x: u64 = 4000;
        let mut min_m: u64 = 1;
        let mut max_m: u64 = 4000;
        let mut min_a: u64 = 1;
        let mut max_a: u64 = 4000;
        let mut min_s: u64 = 1;
        let mut max_s: u64 = 4000;

        for ele in path {
            let count = ele.condition.number;

            match ele.condition.part {
                "x" => {
                    if ele.condition.operator == "<" {
                        max_x = max_x.min(count - 1);
                    } else {
                        min_x = min_x.max(count + 1);
                    }
                }
                "m" => {
                    if ele.condition.operator == "<" {
                        max_m = max_m.min(count - 1);
                    } else {
                        min_m = min_m.max(count + 1);
                    }
                }
                "a" => {
                    if ele.condition.operator == "<" {
                        max_a = max_a.min(count - 1);
                    } else {
                        min_a = min_a.max(count + 1);
                    }
                }
                "s" => {
                    if ele.condition.operator == "<" {
                        max_s = max_s.min(count - 1);
                    } else {
                        min_s = min_s.max(count + 1);
                    }
                }
                _ => panic!("Unexpected part in path"),
            }
        }

        if min_x > max_x || min_m > max_m || min_a > max_a || min_s > max_s {
            continue;
        }

        let sum =
            (max_x - min_x + 1) * (max_m - min_m + 1) * (max_a - min_a + 1) * (max_s - min_s + 1);
        accepted_part_combination += sum;
    }
    accepted_part_combination
}

fn parse_workflows(input: &str) -> IResult<&str, HashMap<&str, Vec<ParsedRule>>> {
    let (input, workflows) = separated_list1(line_ending, parse_workflow)(input)?;

    let mut map = HashMap::new();
    for workflow in workflows {
        map.insert(workflow.name, workflow.rules);
    }

    Ok((input, map))
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
    let (input, part) = alpha1(input)?;
    let (input, operator) = alt((tag("<"), tag(">")))(input)?;
    let (input, number) = complete::u64(input)?;

    Ok((
        input,
        Condition {
            part,
            operator,
            number,
        },
    ))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./test.txt");
        let result = do_magic(input);
        assert_eq!(result, 167409079868000);
    }
}
