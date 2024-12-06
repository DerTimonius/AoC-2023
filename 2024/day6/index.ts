type Field = {
	char: string;
	visited: boolean;
	visitedFrom?: number[][];
};

type Grid = Field[][];

export async function solveDay6(
	type: 'actual' | 'basic' | 'basic_new',
	part: 1 | 2,
): Promise<number> {
	const file = Bun.file(`${import.meta.dir}/${type}.txt`);
	const text = await file.text();

	const grid = text
		.trim()
		.split('\n')
		.map((line) => line.split('').map((char) => ({ char, visited: false })));

	const start = findStart(grid);
	return part === 1 ? solvePart1(grid, start) : solvePart2(grid, start);
}

function solvePart1(grid: Grid, start: number[]): number {
	let sum = 0;
	let [row, col] = start;
	let currIdx = 0;
	const dirs = [
		[-1, 0],
		[0, 1],
		[1, 0],
		[0, -1],
	];

	console.table(grid.map((r) => r.map((x) => x.char)));
	while (isInGrid(grid, row, col)) {
		const [rowDir, colDir] = dirs[currIdx];
		const newRow = row + rowDir;
		const newCol = col + colDir;
		if (!isInGrid(grid, newRow, newCol)) break;
		const nextChar = grid[newRow][newCol].char;

		if (nextChar !== '#') {
			sum += Number(!grid[row][col].visited);
			grid[row][col].visited = true;
			row = newRow;
			col = newCol;
			grid[row][col].char = 'X';
			continue;
		}

		currIdx = (currIdx + 1) % 4;
	}

	// console.table(grid.map((r) => r.map((x) => x.char)));
	return ++sum;
}

function solvePart2(grid: Grid, start: number[]): number {
	solvePart1(grid, start);
	let sum = 0;
	for (let row = 0; row < grid.length; row++) {
		for (let col = 0; col < grid[row].length; col++) {
			if (
				grid[row][col].char !== 'X' ||
				(row === start[0] && col === start[1])
			) {
				continue;
			}

			console.log(`checking at ${row} and ${col}`);
			grid[row][col].char = 'O';
			sum += Number(isInfiniteLoop(grid, start));
			grid[row][col].char = '.';
		}
	}

	return sum;
}

function isInfiniteLoop(gridBase: Grid, start: number[]): boolean {
	let [row, col] = start;
	const grid = JSON.parse(JSON.stringify(gridBase)) as Grid;
	let currIdx = 0;
	const dirs = [
		[-1, 0],
		[0, 1],
		[1, 0],
		[0, -1],
	];

	while (isInGrid(grid, row, col)) {
		const [rowDir, colDir] = dirs[currIdx];
		const newRow = row + rowDir;
		const newCol = col + colDir;
		if (!isInGrid(grid, newRow, newCol)) break;
		const nextChar = grid[newRow][newCol].char;

		if (nextChar !== '#' && nextChar !== 'O') {
			row = newRow;
			col = newCol;
			continue;
		}

		if (
			grid[newRow][newCol].visitedFrom?.length &&
			grid[newRow][newCol].visitedFrom?.some(
				([row, col]) => row === rowDir && col === colDir,
			)
		) {
			return true;
		}

		if (grid[newRow][newCol].visitedFrom) {
			grid[newRow][newCol].visitedFrom?.push([rowDir, colDir]);
		} else {
			grid[newRow][newCol].visitedFrom = [[rowDir, colDir]];
		}
		currIdx = (currIdx + 1) % 4;
	}

	return false;
}

export function isInGrid(grid: Grid, row: number, col: number): boolean {
	return row >= 0 && col >= 0 && row < grid.length && col < grid[row].length;
}

export function findStart(grid: Grid): number[] {
	for (let row = 0; row < grid.length; row++) {
		for (let col = 0; col < grid[row].length; col++) {
			if (grid[row][col].char === '^') {
				return [row, col];
			}
		}
	}

	throw new Error('could not find the start');
}
