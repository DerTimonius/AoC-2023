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
    games.lines().map(|game| validate_game(game)).sum()
}
fn validate_game(game: &str) -> usize {
    let game_info = game.split_vec(": ");
    let games = game_info[1].split_vec("; ");
    let mut max_cubes = HashMap::from([
        (String::from("red"), 0),
        (String::from("blue"), 0),
        (String::from("green"), 0),
    ]);
    for game in games {
        get_max_cube(game, &mut max_cubes);
    }
    max_cubes.values().fold(1, |acc, &value| acc * value)
}

fn get_max_cube(subset: &str, max_cube: &mut HashMap<String, usize>) {
    let cubes = subset.split_vec(", ");
    for cube in cubes {
        let cube_info = cube.split_vec(" ");
        let value = cube_info[0].parse::<usize>().unwrap_or(0);
        let color = cube_info[1].to_owned();
        if value > *max_cube.get(&color).unwrap() {
            max_cube.insert(color, value.to_owned());
        }
    }
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
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 2286);
    }
}
