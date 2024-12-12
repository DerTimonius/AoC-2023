import { isInGrid } from '../utils/isInGrid';

type Plant = {
	area: number;
	perimeter: number;
	val: string;
	sides: number;
};

export async function solveDay12(
	type: 'actual' | 'basic' | 'basic_new' | 'basic1' | 'basic2',
	part: 1 | 2,
): Promise<number> {
	const file = Bun.file(`${import.meta.dir}/${type}.txt`);
	const text = await file.text();
	const grid = text.split('\n').map((l) => l.split(''));

	console.table(grid);
	const plants = getPlants(grid);
	return part === 1 ? solvePart1(grid, plants) : solvePart2(grid, plants);
}

const directions = [
	[-1, 0],
	[0, 1],
	[1, 0],
	[0, -1],
];

function dfs(
	grid: string[][],
	row: number,
	col: number,
	target: string,
	visited: Set<string>,
) {
	const result = { area: 0, perimeter: 0, sides: 0, cells: [] as number[][] };

	function explore(row: number, col: number) {
		if (!isInGrid(grid, row, col)) {
			return;
		}
		if (visited.has(`${row}-${col}`) || grid[row][col] !== target) {
			return;
		}

		visited.add(`${row}-${col}`);
		result.cells.push([row, col]);

		result.area++;
		for (const [rowDir, colDir] of directions) {
			explore(row + rowDir, col + colDir);
		}
	}

	explore(row, col);
	result.perimeter = countPerimeters(grid, result.cells, target);
	result.sides = countCorners(grid, result.cells, target);

	return result;
}

function countCorners(grid: string[][], cells: number[][], target: string) {
	const corners = new Set<string>();

	const isTarget = (row: number, col: number): boolean => {
		if (!isInGrid(grid, row, col)) {
			return false;
		}
		return grid[row][col] === target;
	};

	for (const [row, col] of cells) {
		if (!isTarget(row - 1, col) && !isTarget(row, col - 1)) {
			if (
				isTarget(row - 1, col - 1) &&
				cells.some(([r, c]) => r === row - 1 && c === col - 1)
			) {
				corners.add(`${row - 1},${col - 1}-${row},${col}-tl`);
			} else {
				corners.add(`${row - 1},${col - 1}-${row},${col}`);
			}
		}

		if (!isTarget(row - 1, col) && !isTarget(row, col + 1)) {
			if (
				isTarget(row - 1, col + 1) &&
				cells.some(([r, c]) => r === row - 1 && c === col + 1)
			) {
				corners.add(`${row - 1},${col + 1}-${row},${col}-tr`);
			} else {
				corners.add(`${row - 1},${col + 1}-${row},${col}`);
			}
		}

		if (!isTarget(row + 1, col) && !isTarget(row, col - 1)) {
			if (
				isTarget(row + 1, col - 1) &&
				cells.some(([r, c]) => r === row + 1 && c === col - 1)
			) {
				corners.add(`${row},${col}-${row + 1},${col - 1}-bl`);
			} else {
				corners.add(`${row},${col}-${row + 1},${col - 1}`);
			}
		}

		if (!isTarget(row + 1, col) && !isTarget(row, col + 1)) {
			if (
				isTarget(row + 1, col + 1) &&
				cells.some(([r, c]) => r === row + 1 && c === col + 1)
			) {
				corners.add(`${row},${col}-${row + 1},${col + 1}-br`);
			} else {
				corners.add(`${row},${col}-${row + 1},${col + 1}`);
			}
		}

		if (
			isTarget(row, col - 1) &&
			!isTarget(row - 1, col) &&
			isTarget(row - 1, col - 1)
		) {
			corners.add(`${row - 1},${col - 1}-${row},${col}`);
		}
		if (
			isTarget(row, col + 1) &&
			!isTarget(row - 1, col) &&
			isTarget(row - 1, col + 1)
		) {
			corners.add(`${row - 1},${col + 1}-${row},${col}`);
		}
		if (
			isTarget(row, col - 1) &&
			!isTarget(row + 1, col) &&
			isTarget(row + 1, col - 1)
		) {
			corners.add(`${row},${col}-${row + 1},${col - 1}`);
		}
		if (
			isTarget(row, col + 1) &&
			!isTarget(row + 1, col) &&
			isTarget(row + 1, col + 1)
		) {
			corners.add(`${row},${col}-${row + 1},${col + 1}`);
		}

		if (
			!isTarget(row, col - 1) &&
			isTarget(row - 1, col) &&
			isTarget(row - 1, col - 1)
		) {
			corners.add(`${row - 1},${col - 1}-${row},${col}`);
		}
		if (
			!isTarget(row, col + 1) &&
			isTarget(row - 1, col) &&
			isTarget(row - 1, col + 1)
		) {
			corners.add(`${row - 1},${col + 1}-${row},${col}`);
		}
		if (
			!isTarget(row, col - 1) &&
			isTarget(row + 1, col) &&
			isTarget(row + 1, col - 1)
		) {
			corners.add(`${row},${col}-${row + 1},${col - 1}`);
		}
		if (
			!isTarget(row, col + 1) &&
			isTarget(row + 1, col) &&
			isTarget(row + 1, col + 1)
		) {
			corners.add(`${row},${col}-${row + 1},${col + 1}`);
		}
	}

	return corners.size;
}

function countPerimeters(grid: string[][], cells: number[][], target: string) {
	let edges = 0;
	for (const [row, col] of cells) {
		for (const [rowDir, colDir] of directions) {
			const newRow = row + rowDir;
			const newCol = col + colDir;

			if (!isInGrid(grid, newRow, newCol) || grid[newRow][newCol] !== target) {
				edges++;
			}
		}
	}

	return edges;
}

function getPlants(grid: string[][]): Plant[] {
	const plants = [] as Plant[];
	const visited = new Set<string>();
	for (let row = 0; row < grid.length; row++) {
		for (let col = 0; col < grid[row].length; col++) {
			if (visited.has(`${row}-${col}`)) continue;

			const currVal = grid[row][col];
			const { area, perimeter, sides } = dfs(grid, row, col, currVal, visited);
			plants.push({ area, perimeter, sides, val: currVal });
		}
	}
	return plants;
}

function solvePart1(grid: string[][], plants: Plant[]): number {
	return plants.reduce((acc, plant) => acc + plant.perimeter * plant.area, 0);
}

function solvePart2(grid: string[][], plants: Plant[]): number {
	return plants.reduce((acc, plant) => acc + plant.sides * plant.area, 0);
}
