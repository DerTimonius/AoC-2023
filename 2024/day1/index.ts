export async function solveDay1(
	type: 'actual' | 'basic',
	part: 1 | 2,
): Promise<number> {
	const file = Bun.file(`${import.meta.dir}/${type}.txt`);
	const text = await file.text();

	const lines = text.split('\n');
	const left = [] as number[];
	const right = [] as number[];
	for (const line of lines) {
		const nums = parseLine(line);

		if (nums) {
			left.push(Number(nums[0]));
			right.push(Number(nums[1]));
		}
	}

	return part === 1 ? solvePart1(left, right) : solvePart2(left, right);
}

function solvePart1(nums1: number[], nums2: number[]): number {
	const left = nums1.toSorted((a, b) => a - b);
	const right = nums2.toSorted((a, b) => a - b);

	if (left.length !== right.length) {
		throw new Error('Arrays are of different length');
	}

	let distance = 0;

	for (let i = 0; i < left.length; i++) {
		distance += Math.abs(right[i] - left[i]);
	}

	return distance;
}

function solvePart2(left: number[], right: number[]): number {
	const rightMap: Record<number, number> = {};
	for (const num of right) {
		if (!rightMap[num]) {
			rightMap[num] = 0;
		}
		rightMap[num] += 1;
	}

	let similarity = 0;
	for (const num of left) {
		const times = rightMap[num] ?? 0;
		similarity += num * times;
	}
	return similarity;
}

export function parseLine(line: string) {
	return line.match(/\d+/g);
}
