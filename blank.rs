fn main() {
    // let input = include_str!("./input.txt");
    let input = include_str!("./test.txt");
    let result = do_magic(input);
    println!("{}", result);
}

fn do_magic(input: &str) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./test.txt");
        let result = do_magic(input);
        assert_eq!(result, 142);
    }
}
