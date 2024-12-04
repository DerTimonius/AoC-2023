export async function solveDay4(
	type: 'actual' | 'basic' | 'basic_new',
	part: 1 | 2,
): Promise<number> {
	const file = Bun.file(`${import.meta.dir}/${type}.txt`);
	const text = await file.text();

	const lines = text.trim().split('\n');
	const grid = createGrid(lines);

	return part === 1 ? solvePart1(grid) : solvePart2(grid);
}

function solvePart1(grid: string[][]): number {
	let sum = 0;

	for (let row = 0; row < grid.length; row++) {
		for (let col = 0; col < grid[row].length; col++) {
			if (grid[row][col] !== 'X') continue;
			for (const direction of directions) {
				let newRow = row + direction[0];
				let newCol = col + direction[1];

				const checkRow = (char: string) => {
					if (
						newRow < 0 ||
						newRow >= grid.length ||
						newCol < 0 ||
						newCol >= grid[row].length
					)
						return false;

					if (grid[newRow][newCol] !== char) return false;
					return true;
				};

				if (!checkRow('M')) continue;

				newRow += direction[0];
				newCol += direction[1];

				if (!checkRow('A')) continue;

				newRow += direction[0];
				newCol += direction[1];
				if (checkRow('S')) {
					sum += 1;
				}
			}
		}
	}
	return sum;
}

function solvePart2(grid: string[][]): number {
	let sum = 0;

	for (let row = 0; row < grid.length; row++) {
		for (let col = 0; col < grid[row].length; col++) {
			const char = grid[row][col];
			if (char === 'X' || char === 'A') continue;
			let direction = [1, 1];
			let newRow = row + direction[0];
			let newCol = col + direction[1];

			const checkRow = (char: string) => {
				if (
					newRow < 0 ||
					newRow >= grid.length ||
					newCol < 0 ||
					newCol >= grid[row].length
				)
					return false;

				if (grid[newRow][newCol] !== char) return false;
				return true;
			};

			if (!checkRow('A')) continue;

			newRow += direction[0];
			newCol += direction[1];
			if ((char === 'M' && checkRow('S')) || (char === 'S' && checkRow('M'))) {
				direction = [1, -1];
				newRow = row;
				newCol = col + 2;

				const newChar = grid[newRow][newCol];
				if (newChar === 'X' || newChar === 'A') continue;
				const checkRow = (char: string) => {
					if (
						newRow < 0 ||
						newRow >= grid.length ||
						newCol < 0 ||
						newCol >= grid[row].length
					)
						return false;

					if (grid[newRow][newCol] !== char) return false;
					return true;
				};
				newRow += direction[0];
				newCol += direction[1];

				if (!checkRow('A')) continue;

				newRow += direction[0];
				newCol += direction[1];
				if (
					(newChar === 'M' && checkRow('S')) ||
					(newChar === 'S' && checkRow('M'))
				) {
					sum += 1;
				}
			}
		}
	}
	return sum;
}

export function createGrid(lines: string[]): string[][] {
	return lines.map((l) => l.split(''));
}

const directions = [
	[-1, -1],
	[-1, 0],
	[-1, 1],
	[0, 1],
	[1, 1],
	[1, 0],
	[1, -1],
	[0, -1],
];
