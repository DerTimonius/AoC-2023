export async function solveDay3(
	type: 'actual' | 'basic' | 'basic_new',
	_part: 1 | 2,
): Promise<number> {
	const file = Bun.file(`${import.meta.dir}/${type}.txt`);
	const text = await file.text();

	const line = text.trim();
	const l = parseLine(line);
	const m = l?.map((x) => parseMult(x)) ?? null;
	if (!m) {
		throw new Error('should parse mults');
	}
	const lines = m.map((x) => x?.map((y) => Number(y)));

	return solve(lines.filter(Boolean));
}

function solve(lines: number[][]): number {
	return lines.reduce((acc, nums) => {
		return acc + nums[0] * nums[1];
	}, 0);
}

export function parseLine(line: string) {
	const newLine = line.replaceAll(/don't\(\).+?do\(\)/g, '');
	return newLine.match(/mul\(\d+,\d+\)/g);
}

export function parseMult(str: string) {
	return str.match(/\d+/g);
}

export function checkSafety(nums: number[]): boolean {
	const diffs = nums.map((num, i) => num - nums[i - 1]).slice(1);
	return (
		diffs.every((diff) => diff < 0 && diff >= -3) ||
		diffs.every((diff) => diff > 0 && diff <= 3)
	);
}
