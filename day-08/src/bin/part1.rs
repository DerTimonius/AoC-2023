use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, multispace1},
    multi::separated_list1,
    sequence::{preceded, separated_pair, pair},
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
    // let input = include_str!("./test.txt");
    let result = do_magic(input);
    println!("{}", result);
}

fn do_magic(input: &str) -> u32 {
    let (_, (directions, nodes)) = parse_plan(input).expect("should parse without errors");
    let mut current_node = nodes.iter().find(|node| node.name == "AAA").unwrap().clone();
    let mut steps = 0;

    let binding = directions.repeat(100);
    let mut directions = binding.as_str();

    while directions.len() > 0 {
        let step = directions.chars().nth(0).unwrap();
        steps += 1;
        if step == 'R' {
            current_node = nodes.iter().find(|node| node.name == current_node.direction.right).unwrap().clone()
        } else {
            current_node = nodes.iter().find(|node| node.name == current_node.direction.left).unwrap().clone()
        }

        if current_node.name == "ZZZ" {
            break
        }
        directions = &directions[1..];
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
    let (input, (name, direction)) = separated_pair(alpha1, tag(" = "), direction)(input)?;

    Ok((input, Node { name, direction }))
}

fn direction(input: &str) -> IResult<&str, Direction> {
    let (input, (left, right)) = preceded(
        tag("("),
        separated_pair(alpha1, pair(tag(","), multispace1), alpha1),
    )(input)?;

    let input = &input[1..input.len()];

    Ok((input, Direction { left, right }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = include_str!("./test.txt");
        let result = do_magic(input);
        assert_eq!(result, 6);
    }
}
