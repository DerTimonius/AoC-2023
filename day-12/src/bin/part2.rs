#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Spring {
    Unknown,
    Damaged,
    Operational,
}

fn parse(input: &str) -> impl Iterator<Item = (Vec<Spring>, Vec<usize>)> + '_ {
    input.lines().map(|line| {
        let (springs, counts) = line.split_once(' ').unwrap();
        let springs: Vec<Spring> = springs
            .chars()
            .map(|c| match c {
                '.' => Spring::Operational,
                '#' => Spring::Damaged,
                '?' => Spring::Unknown,
                _ => panic!("there should be no other characters"),
            })
            .collect();
        let counts: Vec<usize> = counts.split(',').filter_map(|s| s.parse().ok()).collect();

        (springs, counts)
    })
}

fn count_possible_arangements(mut springs: Vec<Spring>, counts: Vec<usize>) -> u64 {
    springs.push(Spring::Operational);
    let mut cache = vec![vec![None; springs.len()]; counts.len()];
    count_possible_arangements_inner(&springs, &counts, &mut cache)
}

fn count_possible_arangements_inner(
    springs: &[Spring],
    counts: &[usize],
    cache: &mut [Vec<Option<u64>>],
) -> u64 {
    if counts.is_empty() {
        return if springs.contains(&Spring::Damaged) {
            0
        } else {
            1
        };
    }
    if springs.len() < counts.iter().sum::<usize>() + counts.len() {
        return 0;
    }
    if let Some(cached) = cache[counts.len() - 1][springs.len() - 1] {
        return cached;
    }
    let mut arangements = 0;
    if springs[0] != Spring::Damaged {
        arangements += count_possible_arangements_inner(&springs[1..], counts, cache);
    }
    let next_group_size = counts[0];
    if !springs[..next_group_size].contains(&Spring::Operational)
        && springs[next_group_size] != Spring::Damaged
    {
        arangements +=
            count_possible_arangements_inner(&springs[next_group_size + 1..], &counts[1..], cache);
    }
    cache[counts.len() - 1][springs.len() - 1] = Some(arangements);
    arangements
}

fn do_magic(input: &str) -> u64 {
    parse(input)
        .map(|(mut springs, mut counts)| {
            springs = springs
                .iter()
                .copied()
                .chain([Spring::Unknown])
                .cycle()
                .take(springs.len() * 5 + 4)
                .collect();
            counts = counts
                .iter()
                .copied()
                .cycle()
                .take(counts.len() * 5)
                .collect();

            count_possible_arangements(springs, counts)
        })
        .sum()
}

fn main() {
    let input = include_str!("./input.txt");
    // let input = include_str!("./test.txt");
    let result = do_magic(input);
    println!("result: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_works() {
        let input = include_str!("./test.txt");
        let result = do_magic(input);
        assert_eq!(result, 525152);
    }
}
