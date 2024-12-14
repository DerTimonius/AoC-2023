import { isInGrid } from '../utils/isInGrid';

type Robot = {
	position: {
		row: number;
		col: number;
	};
	velocity: {
		row: number;
		col: number;
	};
};

export async function solveDay14(
	type: 'actual' | 'basic',
	part: 1 | 2,
): Promise<number> {
	const file = Bun.file(`${import.meta.dir}/${type}.txt`);
	const text = await file.text();
	const bathroomDimensions = {
		columns: type === 'actual' ? 101 : 11,
		rows: type === 'actual' ? 103 : 7,
	};
	const bathroomGrid = Array.from({ length: bathroomDimensions.rows }, () =>
		Array.from({ length: bathroomDimensions.columns }, () => '.'),
	);

	const robots = text
		.split('\n')
		.filter(Boolean)
		.map((l) => parseRobot(l));

	return part === 1
		? solvePart1(bathroomGrid, robots)
		: solvePart2(bathroomGrid, robots);
}

export function parseRobot(line: string): Robot {
	const parsed = line.match(
		/(?<px>\d+),(?<py>\d+).*=(?<vx>-?\d+),(?<vy>-?\d+)/,
	);
	if (!parsed || !parsed.groups) {
		throw new Error(`could not parse line: ${line}`);
	}

	return {
		position: {
			col: Number.parseInt(parsed.groups.px),
			row: Number.parseInt(parsed.groups.py),
		},
		velocity: {
			col: Number.parseInt(parsed.groups.vx),
			row: Number.parseInt(parsed.groups.vy),
		},
	};
}

export function moveRobot(
	robot: Robot,
	times: number,
	grid: (string | number)[][],
) {
	const newRowPos = (row: number): number => {
		if (row < 0) {
			return grid.length + row;
		}
		if (row === grid.length) {
			return 0;
		}
		if (row > grid.length) {
			return row - grid.length;
		}
		return row;
	};

	const newColPos = (col: number): number => {
		if (col < 0) {
			return grid[0].length + col;
		}
		if (col === grid[0].length) {
			return 0;
		}
		if (col > grid[0].length) {
			return col - grid[0].length;
		}
		return col;
	};

	for (let i = 0; i < times; i++) {
		const newRow = robot.position.row + robot.velocity.row;
		const newCol = robot.position.col + robot.velocity.col;
		// console.log({ i });
		// console.log('robot velocity: ', robot.velocity);
		// console.log(
		// 	'previous position: ',
		// 	robot.position.row,
		// 	'-',
		// 	robot.position.col,
		// );
		// console.log({ newRow });
		// console.log({ newCol });
		robot.position.row = newRowPos(newRow);
		robot.position.col = newColPos(newCol);
		if (!isInGrid(grid, robot.position.row, robot.position.col)) {
			throw new Error(
				`found an invalid robot position: ${robot.position.row} - ${robot.position.col}`,
			);
		}
	}
}

function placeRobot(
	grid: (string | number)[][],
	{ position: { row, col } }: Robot,
	char?: string,
) {
	if (!isInGrid(grid, row, col)) {
		throw new Error('found an invalid robot position');
	}
	if (char) {
		grid[row][col] = char;
	} else if (typeof grid[row][col] === 'number') {
		grid[row][col]++;
	} else {
		grid[row][col] = 1;
	}
}

function calculateGrid(grid: (string | number)[][]): number {
	return grid.reduce(
		(acc, row) =>
			acc +
			row.reduce<number>(
				(accu, cell) => (typeof cell === 'number' ? accu + cell : accu),
				0,
			),
		0,
	);
}

function solvePart1(grid: (string | number)[][], robots: Robot[]): number {
	for (const robot of robots) {
		moveRobot(robot, 100, grid);
		placeRobot(grid, robot);
	}

	const firstQuadrant = grid
		.slice(0, Math.floor(grid.length / 2))
		.map((l) => l.slice(0, Math.floor(grid[0].length / 2)));
	const secondQuadrant = grid
		.slice(Math.ceil(grid.length / 2))
		.map((l) => l.slice(0, Math.floor(grid[0].length / 2)));
	const thirdQuadrant = grid
		.slice(0, Math.floor(grid.length / 2))
		.map((l) => l.slice(Math.ceil(grid[0].length / 2)));
	const fourthQuadrant = grid
		.slice(Math.ceil(grid.length / 2))
		.map((l) => l.slice(Math.ceil(grid[0].length / 2)));

	// console.table(grid)

	return [firstQuadrant, secondQuadrant, thirdQuadrant, fourthQuadrant].reduce(
		(acc, quadrant) => acc * calculateGrid(quadrant),
		1,
	);
}

function solvePart2(grid: (string | number)[][], robots: Robot[]): number {
	let x = 1;
	while (true) {
		for (const robot of robots) {
			moveRobot(robot, 1, grid);
			placeRobot(grid, robot, 'X');
		}
		if (checkGrid(grid)) {
			console.table(grid);
			return x;
		}
		resetGrid(grid);
		x++;
	}
}

function checkGrid(grid: (string | number)[][]) {
	const str = grid.map((row) => row.join('')).join('');
	return str.includes('XXXXXXXXXXXXXX');
}

function resetGrid(grid: (string | number)[][]) {
	for (let i = 0; i < grid.length; i++) {
		for (let j = 0; j < grid[i].length; j++) {
			grid[i][j] = '.';
		}
	}
}
