use itertools::Itertools;

#[derive(Debug)]
enum MirrorPosition {
    Vertical(usize),
    Horizontal(usize)
}

fn main() {
    let input = include_str!("./input.txt");
    // let input = include_str!("./test.txt");
    let result = do_magic(input);
    println!("result: {}", result);
}

fn do_magic(input: &str) -> usize {
    let result =
        input.split("\n\n").flat_map(find_mirror).fold(
            0usize,
            |mut acc, item| match item {
                MirrorPosition::Horizontal(num) => {
                    acc += 100 * num;
                    acc
                }
                MirrorPosition::Vertical(num) => {
                    acc += num;
                    acc
                }
            },
        );

    result
}

fn find_mirror(input: &str) -> Option<MirrorPosition> {
    find_vertical_mirror(input)
        .or(find_horizontal_mirror(input))
}

fn find_vertical_mirror(input: &str) -> Option<MirrorPosition> {
    let mut columns_iter_collection = input
        .lines()
        .map(|line| line.chars())
        .collect::<Vec<_>>();
    let columns = std::iter::from_fn(move || {
        let mut items = vec![];
        for iter in &mut columns_iter_collection {
            match iter.next() {
                Some(item) => {
                    items.push(item);
                }
                None => return None,
            }
        }
        Some(items)
    })
    .collect::<Vec<Vec<char>>>();

    let result = columns
        .iter()
        .enumerate()
        .tuple_windows()
        .filter(|((_, line_a), (_, line_b))| {
            line_a == line_b
        })
        .find_map(|((index_a, _), (index_b, _))| {
            let lines_a =
                (&columns[0..=index_a]).iter().rev();
            let lines_b = (&columns[index_b..]).iter();

            lines_a
                .zip(lines_b)
                .all(|(a, b)| a == b)
                .then_some(index_a + 1)
        });
    result.map(|num| MirrorPosition::Vertical(num))
}

fn find_horizontal_mirror(input: &str) -> Option<MirrorPosition> {
    let lines: Vec<&str> = input.lines().collect();
    let result = input
        .lines()
        .enumerate()
        .tuple_windows()
        .filter(|((_, line_a), (_, line_b))| {
            line_a == line_b
        })
        .find_map(|((index_a, _), (index_b, _))| {
            let lines_a =
                (&lines[0..=index_a]).iter().rev();
            let lines_b = (&lines[index_b..]).iter();

            lines_a
                .zip(lines_b)
                .all(|(a, b)| a == b)
                .then_some(index_a + 1)
        });
    result.map(|num| MirrorPosition::Horizontal(num))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = include_str!("./test.txt");
        let result = do_magic(input);
        assert_eq!(result, 405);
    }
}
