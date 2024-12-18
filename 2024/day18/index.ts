import { isInGrid } from '../utils/isInGrid';

const directions = [
	[-1, 0],
	[0, 1],
	[1, 0],
	[0, -1],
];

type Byte = {
	x: number;
	y: number;
};
export async function solveDay18(
	type: 'actual' | 'basic' | 'basic_new',
	part: 1 | 2,
): Promise<number> {
	const file = Bun.file(`${import.meta.dir}/${type}.txt`);
	const text = await file.text();

	const grid = Array.from({ length: type === 'basic' ? 7 : 71 }, () =>
		Array.from({ length: type === 'basic' ? 7 : 71 }, () => '.'),
	);

	const bytes = text
		.split('\n')
		.filter(Boolean)
		.map((line) => parseByte(line));

	return part === 1
		? solvePart1(grid, bytes, type === 'basic' ? 12 : 1024)
		: solvePart2(grid, bytes, type === 'basic' ? 12 : 1024);
}

function parseByte(text: string): Byte {
	const parsed = text.split(',').map(Number);
	if (parsed.length !== 2) {
		throw new Error('incorrect length of parsed');
	}
	return {
		x: parsed[0],
		y: parsed[1],
	};
}

function findShortestPath(grid: string[][]): { steps: number; path: string[] } {
	// Find start and end positions

	const start: [number, number] = [0, 0];
	const end: [number, number] = [grid.length - 1, grid[0].length - 1];

	const directions = [
		[0, 1],
		[1, 0],
		[0, -1],
		[-1, 0],
	];
	const queue: Array<[number, number, number]> = [[start[0], start[1], 0]];
	const visited = new Set<string>();
	const parent = new Map<string, string>();

	visited.add(`${start[0]},${start[1]}`);

	while (queue.length > 0) {
		const [row, col, steps] = queue.shift()!;
		const currentKey = `${row},${col}`;

		if (row === end[0] && col === end[1]) {
			// Reconstruct path
			const path: string[] = [];
			let current = currentKey;

			while (current) {
				path.unshift(current);
				current = parent.get(current) || '';
			}

			return {
				steps,
				path,
			};
		}

		for (const [dr, dc] of directions) {
			const newRow = row + dr;
			const newCol = col + dc;
			const newKey = `${newRow},${newCol}`;

			if (
				newRow >= 0 &&
				newRow < grid.length &&
				newCol >= 0 &&
				newCol < grid[0].length &&
				grid[newRow][newCol] !== '#' &&
				!visited.has(newKey)
			) {
				visited.add(newKey);
				parent.set(newKey, currentKey);
				queue.push([newRow, newCol, steps + 1]);
			}
		}
	}

	return { steps: -1, path: [] }; // No path found
}

function solvePart1(grid: string[][], bytes: Byte[], cutoff: number): number {
	for (const { x, y } of bytes.slice(0, cutoff)) {
		grid[y][x] = '#';
	}
	const { steps, path } = findShortestPath(grid);
	paintPath(grid, path);
	// console.table(grid);
	return steps;
}

function solvePart2(grid: string[][], bytes: Byte[], cutoff: number): number {
	for (const { x, y } of bytes.slice(0, cutoff)) {
		grid[y][x] = '#';
	}
	for (let i = cutoff; i < bytes.length; i++) {
		const { x, y } = bytes[i];
		grid[y][x] = '#';
		if (findShortestPath(grid).steps === -1) {
			console.log(`${x},${y}`);
			break;
		}
	}

	return 0;
}

function paintPath(grid: string[][], path: string[]) {
	for (const cell of path) {
		const [row, col] = cell.split(',').map(Number);
		grid[row][col] = 'O';
	}
}
