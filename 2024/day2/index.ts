export async function solveDay2(
	type: 'actual' | 'basic',
	part: 1 | 2,
): Promise<number> {
	const file = Bun.file(`${import.meta.dir}/${type}.txt`);
	const text = await file.text();

	const lines = text
		.trim()
		.split('\n')
		.map((line) => {
			const l = parseLine(line);
			return l?.map((x) => Number(x)) ?? null;
		});

	return part === 1
		? solvePart1(lines.filter(Boolean))
		: solvePart2(lines.filter(Boolean));
}

function solvePart1(lines: number[][]): number {
	return lines.reduce((acc, nums) => acc + Number(checkSafety(nums)), 0);
}

function solvePart2(lines: number[][]): number {
	return lines.reduce((acc, nums) => {
		for (let i = 0; i < nums.length; i++) {
			if (checkSafety(nums.toSpliced(i, 1))) {
				return acc + 1;
			}
		}
		return acc;
	}, 0);
}

export function parseLine(line: string) {
	return line.match(/\d+/g);
}

export function checkSafety(nums: number[]): boolean {
	let direction: Direction | undefined;
	for (let i = 0; i < nums.length - 1; i++) {
		const diff = nums[i] - nums[i + 1];
		const newDir: Direction = diff > 0 ? 'asc' : 'desc';
		if (
			(direction && newDir !== direction) ||
			Math.abs(diff) > 3 ||
			diff === 0
		) {
			return false;
		}

		direction = newDir;
	}

	return true;
}

type Direction = 'asc' | 'desc';
