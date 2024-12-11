export async function solveDay11(
	type: 'actual' | 'basic' | 'basic_new',
	part: 1 | 2,
): Promise<number> {
	const file = Bun.file(`${import.meta.dir}/${type}.txt`);
	const text = await file.text();
	const nums = text.replace('\n', '').split(' ');

	return part === 1 ? solve(nums, 25) : solve(nums, 75);
}

function solve(nums: string[], blinks: number): number {
	const memo = new Map<string, number>();

	function predictGrowth(num: string, remainingSteps: number): number {
		const key = `${num}-${remainingSteps}`;

		// biome-ignore lint/style/noNonNullAssertion: existence was checked
		if (memo.has(key)) return memo.get(key)!;
		if (remainingSteps === 0) return 1;

		let count = 1;
		if (num === '0') {
			count = predictGrowth('1', remainingSteps - 1);
		} else if (num.length % 2 === 0) {
			const firstHalf = Number.parseInt(
				num.substring(0, num.length / 2),
			).toString();
			const secondHalf = Number.parseInt(
				num.substring(num.length / 2),
			).toString();
			count =
				predictGrowth(firstHalf, remainingSteps - 1) +
				predictGrowth(secondHalf, remainingSteps - 1);
		} else {
			count = predictGrowth(
				(Number.parseInt(num) * 2024).toString(),
				remainingSteps - 1,
			);
		}

		memo.set(key, count);
		return count;
	}

	return nums.reduce((total, num) => total + predictGrowth(num, blinks), 0);
}
