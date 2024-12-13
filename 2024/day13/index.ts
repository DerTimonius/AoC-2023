type Game = {
	buttonA: { x: number; y: number };
	buttonB: { x: number; y: number };
	prize: { x: number; y: number };
};

type Solution = {
	buttonAPresses: number;
	buttonBPresses: number;
};

export async function solveDay13(
	type: 'actual' | 'basic' | 'basic_new' | 'basic1' | 'basic2',
	part: 1 | 2,
): Promise<number> {
	const file = Bun.file(`${import.meta.dir}/${type}.txt`);
	const text = await file.text();
	const blocks = text.split('\n\n');
	const games = blocks.map((block) => parseBlock(block));

	return part === 1 ? solvePart1(games) : solvePart2(games);
}

export function parseBlock(block: string): Game {
	const lines = block.split('\n');
	const buttonA = lines[0].match(/.*X\+(?<x>\d+), Y\+(?<y>\d+)/);
	const buttonB = lines[1].match(/.*X\+(?<x>\d+), Y\+(?<y>\d+)/);
	const prize = lines[2].match(/.*X=(?<x>\d+), Y=(?<y>\d+)/);

	if (!buttonA?.groups || !buttonB?.groups || !prize?.groups) {
		throw new Error('could not parse block');
	}

	return {
		buttonA: {
			x: Number.parseInt(buttonA.groups.x),
			y: Number.parseInt(buttonA.groups.y),
		},
		buttonB: {
			x: Number.parseInt(buttonB.groups.x),
			y: Number.parseInt(buttonB.groups.y),
		},
		prize: {
			x: Number.parseInt(prize.groups.x),
			y: Number.parseInt(prize.groups.y),
		},
	};
}

export function calculatePresses(game: Game, max: number): Solution | null {
	const maxTries = Math.max(
		Math.min(
			max,
			Math.floor(game.prize.x / Math.min(game.buttonA.x, game.buttonB.x)),
		),
		Math.min(
			max,
			Math.floor(game.prize.y / Math.min(game.buttonA.y, game.buttonB.y)),
		),
	);

	for (let a = 0; a <= maxTries; a++) {
		for (let b = 0; b < maxTries; b++) {
			const sumX = a * game.buttonA.x + b * game.buttonB.x;
			const sumY = a * game.buttonA.y + b * game.buttonB.y;

			if (sumX === game.prize.x && sumY === game.prize.y) {
				return {
					buttonAPresses: a,
					buttonBPresses: b,
				};
			}
		}
	}

	return null;
}

function calculateEfficiently({
	buttonA,
	buttonB,
	prize,
}: Game): Solution | null {
	const determinant = buttonA.x * buttonB.y - buttonA.y * buttonB.x;

	if (!determinant) {
		return null;
	}

	const a = (-buttonB.x * prize.y + buttonB.y * prize.x) / determinant;
	const b = (buttonA.x * prize.y - buttonA.y * prize.x) / determinant;

	if (a === Math.floor(a) && b === Math.floor(b) && a > 0 && b > 0) {
		return { buttonBPresses: b, buttonAPresses: a };
	}

	return null;
}

function solvePart1(games: Game[]): number {
	return games.reduce((acc, game) => {
		const result = calculateEfficiently(game);
		if (result) {
			return acc + result.buttonAPresses * 3 + result.buttonBPresses * 1;
		}

		return acc;
	}, 0);
}

function solvePart2(games: Game[]): number {
	return games
		.map((game) => ({
			...game,
			prize: { x: game.prize.x + 10e12, y: game.prize.y + 10e12 },
		}))
		.reduce((acc, game) => {
			const result = calculateEfficiently(game);
			if (result) {
				return acc + result.buttonAPresses * 3 + result.buttonBPresses * 1;
			}

			return acc;
		}, 0);
}
