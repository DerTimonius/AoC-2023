// don't use this! it works, but brute force, would take forever

use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, line_ending, multispace1},
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair},
    IResult,
};

#[derive(Debug, Copy, Clone)]
struct Node<'a> {
    name: &'a str,
    direction: Direction<'a>,
}

#[derive(Debug, Copy, Clone)]
struct Direction<'a> {
    left: &'a str,
    right: &'a str,
}
fn main() {
    let input = include_str!("./input.txt");
    // let input = include_str!("./test2.txt");
    let result = do_magic(input);
    println!("{}", result);
}

fn do_magic<'a>(input: &str) -> u128 {
    let (_, (path, nodes)) = parse_plan(input).expect("should parse without errors");
    let starting_nodes: HashMap<&str, Direction> = nodes
        .iter()
        .filter(|node| node.name.ends_with("A"))
        .map(|node| (node.name, node.direction))
        .collect();
    let mut current_nodes = starting_nodes.clone();
    let nodes: HashMap<&str, Direction> = nodes
        .iter()
        .map(|node| (node.name, node.direction))
        .collect();
    let mut steps: u128 = 0;

    let mut directions = path;


    loop {
        let step = directions.chars().nth(0).unwrap();
        steps += 1;
        if step == 'R' {
            let updated_nodes: HashMap<&str, Direction> = current_nodes
                .iter()
                .map(|(_, directions)| {
                    if let Some(node) = nodes.get(directions.right) {
                        (directions.right, node.clone())
                    } else {
                        panic!("did not find the next node")
                    }
                })
                .collect();

            current_nodes = updated_nodes
        } else {
            let updated_nodes: HashMap<&str, Direction> = current_nodes
                .iter()
                .map(|(_, directions)| {
                    if let Some(node) = nodes.get(directions.left) {
                        (directions.left, node.clone())
                    } else {
                        panic!("did not find the next node")
                    }
                })
                .collect();

            current_nodes = updated_nodes
        }

        if current_nodes.keys().any(|name| name.ends_with("Z")) {
          println!("at step {steps}");
            println!("current_nodes: {:?}", current_nodes);
        }

        if current_nodes.keys().all(|name| name.ends_with("Z")) {
            println!("found it!");
            break;
        }

        if directions.len() == 1 {
          directions = path
        } else {

          directions = &directions[1..];
        }
    }

    steps
}

fn parse_plan<'a>(input: &'a str) -> IResult<&str, (&str, Vec<Node<'a>>)> {
    let (input, plan) = separated_pair(
        alpha1,
        line_ending,
        preceded(line_ending, separated_list1(line_ending, node)),
    )(input)?;

    Ok((input, plan))
}

fn node(input: &str) -> IResult<&str, Node> {
    let (input, (name, direction)) = separated_pair(alphanumeric1, tag(" = "), direction)(input)?;

    Ok((input, Node { name, direction }))
}

fn direction(input: &str) -> IResult<&str, Direction> {
    let (input, (left, right)) = preceded(
        tag("("),
        separated_pair(alphanumeric1, pair(tag(","), multispace1), alphanumeric1),
    )(input)?;

    let input = &input[1..input.len()];

    Ok((input, Direction { left, right }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works() {
        let input = include_str!("./test2.txt");
        let result = do_magic(input);
        assert_eq!(result, 6);
    }
}
