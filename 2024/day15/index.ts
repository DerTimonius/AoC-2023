import { isInGrid } from '../utils/isInGrid';

export async function solveDay15(
	type: 'actual' | 'basic' | 'basic_new' | 'basic1',
	part: 1 | 2,
): Promise<number> {
	const file = Bun.file(`${import.meta.dir}/${type}.txt`);
	const text = await file.text();
	const [gridText, directionsText] = text.split('\n\n');
	const grid = gridText.split('\n').map((row) => row.split(''));
	const directions = directionsText.split('').filter((char) => char !== '\n');

	return part === 1
		? solvePart1(grid, directions)
		: solvePart2(grid, directions);
}

const directions = {
	up: [-1, 0],
	right: [0, 1],
	down: [1, 0],
	left: [0, -1],
} as const;

function getDirection(input: string) {
	switch (input) {
		case '^':
			return directions.up;
		case '<':
			return directions.left;
		case '>':
			return directions.right;
		case 'v':
			return directions.down;
		default:
			throw new Error('unknown direction');
	}
}

function getStartPosition(grid: string[][]) {
	for (let i = 0; i < grid.length; i++) {
		for (let j = 0; j < grid[i].length; j++) {
			if (grid[i][j] === '@') {
				return [i, j];
			}
		}
	}

	throw new Error('start not found');
}

function move(
	grid: string[][],
	[row, col]: number[],
	[rowDir, colDir]: number[],
) {
	const newRow = row + rowDir;
	const newCol = col + colDir;
	if (!isInGrid(grid, newRow, newCol)) {
		throw new Error('should not be out of bounds');
	}

	if (grid[newRow][newCol] === '.') {
		grid[newRow][newCol] = '@';
		grid[row][col] = '.';
		return [newRow, newCol];
	}

	if (grid[newRow][newCol] === '#') {
		return [row, col];
	}

	if (grid[newRow][newCol] === 'O') {
		let posRow = newRow + rowDir;
		let posCol = newCol + colDir;
		while (grid[posRow][posCol] === 'O') {
			posRow += rowDir;
			posCol += colDir;
		}

		if (grid[posRow][posCol] === '#') {
			return [row, col];
		}

		if (grid[posRow][posCol] === '.') {
			grid[posRow][posCol] = 'O';
			grid[newRow][newCol] = '@';
			grid[row][col] = '.';
			return [newRow, newCol];
		}
	}

	if (grid[newRow][newCol] === '[' || grid[newRow][newCol] === ']') {
		const blockLeftCol = grid[newRow][newCol] === '[' ? newCol : newCol - 1;

		const blocksToMove: Array<{ row: number; leftCol: number }> = [];
		const blocksToCheck: Array<{ row: number; leftCol: number }> = [
			{ row: newRow, leftCol: blockLeftCol },
		];
		const checked = new Set<string>();

		while (blocksToCheck.length) {
			// biome-ignore lint/style/noNonNullAssertion: checked length of array
			const currentBlock = blocksToCheck.pop()!;
			const key = `${currentBlock.row},${currentBlock.leftCol}`;
			if (checked.has(key)) continue;

			checked.add(key);

			if (
				!blocksToMove.some(
					(b) =>
						b.row === currentBlock.row && b.leftCol === currentBlock.leftCol,
				)
			) {
				blocksToMove.push(currentBlock);

				const checkRow = currentBlock.row + rowDir;
				const checkCol = currentBlock.leftCol + colDir;

				if (
					grid[checkRow][checkCol] === '[' ||
					grid[checkRow][checkCol] === ']'
				) {
					blocksToCheck.push({
						row: checkRow,
						leftCol: grid[checkRow][checkCol] === '[' ? checkCol : checkCol - 1,
					});
				}

				if (
					grid[checkRow][checkCol + 1] === '[' ||
					grid[checkRow][checkCol + 1] === ']'
				) {
					blocksToCheck.push({
						row: checkRow,
						leftCol:
							grid[checkRow][checkCol + 1] === '[' ? checkCol + 1 : checkCol,
					});
				}
			}
		}

		for (const block of blocksToMove) {
			const targetRow = block.row + rowDir;
			const targetCol = block.leftCol + colDir;

			if (
				grid[targetRow][targetCol] === '#' ||
				grid[targetRow][targetCol + 1] === '#'
			) {
				return [row, col];
			}
		}
		blocksToMove.sort((a, b) => {
			if (rowDir !== 0) {
				return rowDir * (b.row - a.row); // Sort by row when moving vertically
			}
			return colDir * (b.leftCol - a.leftCol); // Sort by column when moving horizontally
		});

		for (const block of blocksToMove) {
			grid[block.row][block.leftCol] = '.';
			grid[block.row][block.leftCol + 1] = '.';
			grid[block.row + rowDir][block.leftCol + colDir] = '[';
			grid[block.row + rowDir][block.leftCol + colDir + 1] = ']';
		}

		grid[newRow][newCol] = '@';
		grid[row][col] = '.';
		return [newRow, newCol];
	}

	// console.log('char: ', grid[newRow][newCol]);
	throw new Error('encountered unknown character');
}

function calculateGrid(grid: string[][], char: string): number {
	return grid.reduce(
		(acc, row, idx) =>
			acc +
			row.reduce((ac, cell, jdx) => {
				const result = cell === char ? 100 * idx + jdx : 0;
				return ac + result;
			}, 0),
		0,
	);
}

function transformGrid(grid: string[][]) {
	return grid.map((row) =>
		row
			.map((cell) => {
				switch (cell) {
					case '#':
						return '##';
					case '.':
						return '..';
					case '@':
						return '@.';
					case 'O':
						return '[]';
					default:
						throw new Error('encountered unknown character');
				}
			})
			.flatMap((c) => c.split('')),
	);
}

function solvePart1(grid: string[][], directions: string[]): number {
	let [row, col] = getStartPosition(grid);
	for (const direction of directions) {
		[row, col] = move(grid, [row, col], getDirection(direction));
	}

	// console.table(grid);
	return calculateGrid(grid, 'O');
}

function solvePart2(prior: string[][], directions: string[]): number {
	const grid = transformGrid(prior);
	let [row, col] = getStartPosition(grid);
	for (const direction of directions) {
		[row, col] = move(grid, [row, col], getDirection(direction));
	}
	// console.table(grid);
	return calculateGrid(grid, '[');
}
