export async function solveDay19(
	type: 'actual' | 'basic' | 'basic_new',
	part: 1 | 2,
): Promise<number> {
	const file = Bun.file(`${import.meta.dir}/${type}.txt`);
	const text = await file.text();

	const { availablePatterns, towels } = parseInput(text);

	return part === 1
		? solvePart1(towels, availablePatterns)
		: solvePart2(towels, availablePatterns);
}

function parseInput(text: string) {
	const [av, t] = text.split('\n\n');
	if (!av || !t) {
		throw new Error('could parse text correctly');
	}

	return {
		availablePatterns: av.split(', '),
		towels: t.split('\n').filter(Boolean),
	};
}

function canCreateTowel(towel: string, patterns: string[]) {
	const towelLength = towel.length;
	const dpArr = Array.from({ length: towelLength + 1 }, () => false);
	dpArr[towelLength] = true;

	for (let i = towelLength - 1; i >= 0; i--) {
		for (const pattern of patterns) {
			if (
				pattern.length + i <= towelLength &&
				towel.substring(i, i + pattern.length) === pattern &&
				dpArr[i + pattern.length]
			) {
				dpArr[i] = true;
				break;
			}
		}
	}

	return dpArr[0];
}

function countTowelCombinations(
	towel: string,
	patterns: string[],
	start = 0,
	memo: Record<number, number> = {},
) {
	if (memo[start]) return memo[start];

	if (start === towel.length) return 1;

	let combinations = 0;
	for (const pattern of patterns) {
		if (
			start + pattern.length <= towel.length &&
			towel.substring(start, start + pattern.length) === pattern
		) {
			combinations += countTowelCombinations(
				towel,
				patterns,
				start + pattern.length,
				memo,
			);
		}
	}

	memo[start] = combinations;
	return combinations;
}

function solvePart1(towels: string[], patterns: string[]): number {
	return towels.filter((towel) => canCreateTowel(towel, patterns)).length;
}

function solvePart2(towels: string[], patterns: string[]): number {
	let sum = 0;
	for (const towel of towels) {
		sum += countTowelCombinations(towel, patterns);
	}
	return sum;
}
