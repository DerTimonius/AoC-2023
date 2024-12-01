fn main() {
    let input = include_str!("./input.txt");
    // let input = include_str!("./test.txt");
    let result = do_magic(input);
    println!("result: {}", result);
}

fn do_magic(input: &str) -> u32 {
    let values = input.split(",").collect::<Vec<&str>>();
    let result: u32 = values.iter().map(|str| get_hash(str)).sum();
    result
}

fn get_hash(input: &str) -> u32 {
    let input = input.trim_end();
    let nums = input
        .chars()
        .into_iter()
        .map(|c| c as u32)
        .collect::<Vec<u32>>();

    let mut current_sum: u32 = 0;
    for num in nums.iter() {
        current_sum += num;
        current_sum *= 17;
        current_sum = current_sum % 256;
    }

    current_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = include_str!("./test.txt");
        let result = do_magic(input);
        assert_eq!(result, 1320);
    }
}
