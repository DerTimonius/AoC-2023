import { isInGrid } from '../utils/isInGrid';

export async function solveDay10(
	type: 'actual' | 'basic' | 'basic_new',
	part: 1 | 2,
): Promise<number> {
	const file = Bun.file(`${import.meta.dir}/${type}.txt`);
	const text = await file.text();
	const grid = text
		.split('\n')
		.map((l) => l.split('').map((x) => Number.parseInt(x)));

	return part === 1 ? solvePart1(grid) : solvePart2(grid);
}

const directions = [
	[-1, 0],
	[0, 1],
	[1, 0],
	[0, -1],
];

function getStarts(grid: number[][]): number[][] {
	const starts = [] as number[][];
	for (let row = 0; row < grid.length; row++) {
		for (let col = 0; col < grid[row].length; col++) {
			if (grid[row][col] === 0) {
				starts.push([row, col]);
			}
		}
	}

	return starts;
}

function dfs(grid: number[][], [row, col]: number[], visited: number[][]) {
	if (!isInGrid(grid, row, col)) {
		return [];
	}

	if (visited.some(([visRow, visCol]) => visRow === row && visCol === col)) {
		return [];
	}

	visited.push([row, col]);

	const goalPositions = [] as number[][];

	if (grid[row][col] === 9) {
		goalPositions.push([row, col]);
	}

	for (const direction of directions) {
		const newRow = row + direction[0];
		const newCol = col + direction[1];
		if (
			isInGrid(grid, newRow, newCol) &&
			grid[newRow][newCol] - grid[row][col] === 1
		) {
			const positions = dfs(grid, [newRow, newCol], visited);
			goalPositions.push(...positions);
		}
	}

	return goalPositions;
}

function dfsPaths(
	grid: number[][],
	[row, col]: number[],
	currentPath: number[][],
) {
	if (!isInGrid(grid, row, col)) {
		return [];
	}

	const newPath = [...currentPath, [row, col]];
	const distinctPaths = [] as number[][][];

	if (grid[row][col] === 9) {
		distinctPaths.push(newPath);
	}

	for (const direction of directions) {
		const newRow = row + direction[0];
		const newCol = col + direction[1];
		if (
			isInGrid(grid, newRow, newCol) &&
			grid[newRow][newCol] - grid[row][col] === 1
		) {
			const paths = dfsPaths(grid, [newRow, newCol], currentPath);
			for (const path of paths) {
				if (isPathUnique(path, distinctPaths)) {
					distinctPaths.push(path);
				}
			}
		}
	}

	return distinctPaths;
}

function solvePart1(grid: number[][]): number {
	const starts = getStarts(grid);
	return starts.reduce((acc, start) => acc + dfs(grid, start, []).length, 0);
}

function solvePart2(grid: number[][]): number {
	const starts = getStarts(grid);
	return starts.reduce(
		(acc, start) => acc + dfsPaths(grid, start, []).length,
		0,
	);
}

function isPathUnique(path: number[][], paths: number[][][]): boolean {
	for (const existingPath of paths) {
		if (!arePathsEqual(path, existingPath)) {
			return false;
		}
	}

	return true;
}

function arePathsEqual(path1: number[][], path2: number[][]) {
	if (path1.length !== path2.length) return false;

	for (let i = 0; i < path1.length; i++) {
		if (path1[i] === path2[i]) return false;
	}

	return true;
}
