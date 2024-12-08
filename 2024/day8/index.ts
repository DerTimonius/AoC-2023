type Location = number[];
type AntennaMap = Map<string | number, { locations: Location[] }>;

export async function solveDay8(
	type: 'actual' | 'basic' | 'basic_new',
	part: 1 | 2,
): Promise<number> {
	const file = Bun.file(`${import.meta.dir}/${type}.txt`);
	const text = await file.text();
	const grid = text.split('\n').map((l) => l.split(''));

	const map = createAntennaMap(grid);
	return part === 1 ? solvePart1(map, grid) : solvePart2(map, grid);
}

export function createAntennaMap(grid: string[][]): AntennaMap {
	const map = new Map() as AntennaMap;

	for (let row = 0; row < grid.length; row++) {
		for (let col = 0; col < grid[row].length; col++) {
			const char = grid[row][col];
			if (char === '.') continue;

			if (map.has(char)) {
				map.get(char)?.locations.push([row, col]);
			} else {
				map.set(char, { locations: [[row, col]] });
			}
		}
	}

	return map;
}

function solvePart1(map: AntennaMap, grid: string[][]): number {
	for (const [_, val] of map) {
		for (let i = 0; i < val.locations.length - 1; i++) {
			for (let j = i + 1; j < val.locations.length; j++) {
				const [a1Row, a1Col] = val.locations[i];
				const [a2Row, a2Col] = val.locations[j];
				const rowDiff = a2Row - a1Row;
				const colDiff = a2Col - a1Col;
				if (isInGrid(grid, a1Row - rowDiff, a1Col - colDiff)) {
					grid[a1Row - rowDiff][a1Col - colDiff] = '#';
				}

				if (isInGrid(grid, a2Row + rowDiff, a2Col + colDiff)) {
					grid[a2Row + rowDiff][a2Col + colDiff] = '#';
				}
			}
		}
	}

	return countNodes(grid);
}

function countNodes(grid: string[][]): number {
	return grid
		.map((l) => l.join(''))
		.join('')
		.replaceAll(/[^#]/g, '').length;
}

function solvePart2(map: AntennaMap, grid: string[][]): number {
	for (const [_, val] of map) {
		for (let i = 0; i < val.locations.length - 1; i++) {
			for (let j = i + 1; j < val.locations.length; j++) {
				let [a1Row, a1Col] = val.locations[i];
				let [a2Row, a2Col] = val.locations[j];
				const rowDiff = a2Row - a1Row;
				const colDiff = a2Col - a1Col;
				while (isInGrid(grid, a1Row, a1Col)) {
					grid[a1Row][a1Col] = '#';
					a1Row = a1Row - rowDiff;
					a1Col = a1Col - colDiff;
				}

				while (isInGrid(grid, a2Row, a2Col)) {
					grid[a2Row][a2Col] = '#';
					a2Row = a2Row + rowDiff;
					a2Col = a2Col + colDiff;
				}
			}
		}
	}

	return countNodes(grid);
}

export function isInGrid(grid: string[][], row: number, col: number): boolean {
	return row >= 0 && col >= 0 && row < grid.length && col < grid[row].length;
}
