type Calibration = {
	result: number;
	nums: number[];
};
const operators = ['+', '*'];

export async function solveDay7(
	type: 'actual' | 'basic' | 'basic_new',
	part: 1 | 2,
): Promise<number> {
	const file = Bun.file(`${import.meta.dir}/${type}.txt`);
	const text = await file.text();

	const calibrations = parseCalibrations(text.split('\n'));

	return part === 1 ? solvePart1(calibrations) : solvePart2(calibrations);
}

export function parseCalibrations(lines: string[]): Calibration[] {
	return lines.map((l) => {
		const nums = l.match(/\d+/g)?.map((x) => Number(x)) ?? [];

		return { result: nums[0], nums: nums.slice(1) };
	});
}

export function testCalibration(
	{ nums, result }: Calibration,
	operations: ((a: number, b: number) => number)[],
): boolean {
	function backtrack(currIdx: number, currResult: number): boolean {
		if (currIdx === nums.length) return currResult === result;

		for (const op of operations) {
			const nextResult =
				currIdx === 0 ? nums[currIdx] : op(currResult, nums[currIdx]);

			if (backtrack(currIdx + 1, nextResult)) {
				return true;
			}
		}

		return false;
	}
	return backtrack(0, 0);
}

function solvePart1(calibrations: Calibration[]): number {
	const operations = [
		(a: number, b: number) => a + b,
		(a: number, b: number) => a * b,
	];
	return calibrations.reduce((acc, cal) => {
		if (testCalibration(cal, operations)) {
			return acc + cal.result;
		}

		return acc;
	}, 0);
}

function solvePart2(calibrations: Calibration[]): number {
	const operations = [
		(a: number, b: number) => a + b,
		(a: number, b: number) => a * b,
		(a: number, b: number) => Number.parseInt(a.toString() + b.toString()),
	];
	return calibrations.reduce((acc, cal) => {
		if (testCalibration(cal, operations)) {
			return acc + cal.result;
		}

		return acc;
	}, 0);
}
