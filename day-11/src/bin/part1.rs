use pathfinding::grid::Grid;

fn main() {
    let input = include_str!("./input.txt");
    // let input = include_str!("./test.txt");
    let result = do_magic(input);
    println!("result: {}", result);
}

fn do_magic(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let nodes = find_nodes(&grid);
    let grid = Grid::from_coordinates(&nodes).unwrap();

    let distances = get_distances(&grid, &nodes);

    println!("nodes: {:?}", nodes.len());
    println!("connections: {:?}", nodes.len() * (nodes.len() - 1) / 2);
    println!("distances: {:?}", distances.len());

    distances.iter().sum()
}

fn get_distances(grid: &Grid, nodes: &Vec<(usize, usize)>) -> Vec<usize> {
    let mut distances = Vec::new();

    for (i, node_i) in nodes.iter().enumerate() {
        for node_j in nodes.iter().skip(i) {
            if node_i != node_j {
                let distance = grid.distance(*node_i, *node_j);
                distances.push(distance);
            }
        }
    }
    distances
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
        let result = do_magic(input);
        assert_eq!(result, 374);
    }
}
