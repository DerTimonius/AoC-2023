use pathfinding::grid::Grid;

fn main() {
    // let input = include_str!("./input.txt");
    let input = include_str!("./test.txt");
    let result = do_magic(input, 10);
    println!("result: {}", result);
}

fn do_magic(input: &str, exponent: isize) -> isize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let nodes = find_nodes(&grid);
    let grid = Grid::from_coordinates(&nodes).unwrap();

    let distances = get_distances(&grid, &nodes, &exponent);

    distances.iter().sum()
}

fn get_distances(grid: &Grid, nodes: &Vec<(usize, usize)>, exponent: &isize) -> Vec<isize> {
    let mut distances = Vec::new();

    for (i, node_i) in nodes.iter().enumerate() {
        for node_j in nodes.iter().skip(i) {
            if node_i != node_j {
                let distance = calculate_distance(&grid, &node_i, &node_j, exponent);
                distances.push(distance);
            }
        }
    }
    distances
}

fn calculate_distance(grid: &Grid, node_i: &(usize, usize), node_j: &(usize, usize), multiplier: &isize) -> isize {
    let empty_rows = (node_i.1 as isize - node_j.1 as isize).abs() as isize - 1;
    let empty_columns = (node_i.0 as isize - node_j.0 as isize).abs() as isize - 1;

    let base_distance = grid.distance(*node_i, *node_j);
    let adjusted_distance = base_distance as isize * multiplier + empty_rows + empty_columns;

    adjusted_distance
}

fn find_nodes(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut nodes = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, &ch) in row.iter().enumerate() {
            if ch == '#' {
                nodes.push((x, y));
            }
        }
    }

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let input = include_str!("./test.txt");
        let result = do_magic(input, 10);
        assert_eq!(result, 1030);
    }
}
