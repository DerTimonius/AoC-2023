use std::collections::HashMap;

trait StrSplitVec {
    fn split_vec(&self, delimiter: &str) -> Vec<&str>;
}

impl StrSplitVec for &str {
    fn split_vec(&self, delimiter: &str) -> Vec<&str> {
        self.split(delimiter).collect()
    }
}

fn main() {
    println!("a weird puzzle");
    let input = include_str!("./input.txt");
    let result = sum_games(input);
    println!("{}", result);
}

fn sum_games(games: &str) -> usize {
    let valid_nums = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    games
        .lines()
        .filter_map(|game| validate_game(game, &valid_nums))
        .collect::<Vec<usize>>()
        .iter()
        .sum()
}
fn validate_game(game: &str, valid_nums: &HashMap<&str, usize>) -> Option<usize> {
    let game_info = game.split_vec(": ");
    let game_number: usize = game_info[0].split_vec(" ")[1]
        .parse::<usize>()
        .ok()
        .unwrap();
    let ind_games = game_info[1].split_vec("; ");
    for subset in ind_games {
        if !validate_subset(subset, valid_nums) {
            return None;
        }
    }
    Some(game_number)
}

fn validate_subset(subset: &str, valid_nums: &HashMap<&str, usize>) -> bool {
    let cubes = subset.split_vec(", ");
    for cube in cubes {
        let cube_info = cube.split_vec(" ");
        if cube_info[0].parse::<usize>().unwrap() > *valid_nums.get(cube_info[1]).unwrap() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let result = sum_games(
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        );
        assert_eq!(result, 8);
    }
}
