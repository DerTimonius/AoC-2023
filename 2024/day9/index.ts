type Block = {
	value: string | number;
	count: number;
	missingDots?: number;
	checked?: boolean;
};

export async function solveDay9(
	type: 'actual' | 'basic' | 'basic_new',
	part: 1 | 2,
): Promise<number> {
	const file = Bun.file(`${import.meta.dir}/${type}.txt`);
	const text = await file.text();
	return part === 1 ? solvePart1(text) : solvePart2(text);
}

function createBlocks(input: string): Block[] {
	const block = [] as Block[];
	let currNum = 0;
	for (let i = 0; i < input.length; i++) {
		const num = Number.parseInt(input[i]);
		if (Number.isNaN(num)) {
			continue;
		}

		if (i % 2) {
			block.push({ value: '.', count: num });
		} else {
			block.push({ value: currNum, count: num });
			currNum++;
		}
	}

	return block;
}

function moveBlocks(input: (string | number)[]) {
	let swapped = [...input];
	for (let i = 0; i < swapped.length; i++) {
		while (swapped.at(-1) === '.') {
			swapped = swapped.slice(0, swapped.length - 1);
		}
		if (swapped[i] === '.') {
			[swapped[i], swapped[swapped.length - 1]] = [
				swapped[swapped.length - 1],
				swapped[i],
			];
		}
	}

	return swapped.filter((x) => typeof x === 'number');
}

function generateBlockArray(input: Block[]): (string | number)[] {
	return input.reduce(
		(acc, val) => {
			let i = 0;
			while (i < val.count) {
				acc.push(val.value);
				i++;
			}
			i = 0;
			while (val.missingDots && i < val.missingDots) {
				acc.push('.');
				i++;
			}
			return acc;
		},
		[] as (string | number)[],
	);
}

function calculateBlock(nums: (string | number)[]): number {
	return nums.reduce<number>(
		(acc, val, idx) => (typeof val === 'number' ? acc + val * idx : acc),
		0,
	);
}

function solvePart1(input: string): number {
	const blocks = createBlocks(input);

	const moved = moveBlocks(generateBlockArray(blocks));

	return calculateBlock(moved);
}

function solvePart2(input: string): number {
	const blocks = createBlocks(input);
	const moved = defragmentBlocks(blocks);
	return calculateBlock(moved);
}

function defragmentBlocks(input: Block[]): (string | number)[] {
	while (
		input.findLastIndex((block) => !block.checked && block.value !== '.')
	) {
		const i = input.findLastIndex(
			(block) => !block.checked && block.value !== '.',
		);
		input[i].checked = true;
		for (let j = 0; j < i; j++) {
			if (input[j].value !== '.') continue;

			const diff = input[i].count - input[j].count;
			if (diff > 0) continue;

			if (diff === 0) {
				[input[i], input[j]] = [input[j], input[i]];
				break;
			}

			if (diff < 0) {
				const dots = input[j];
				input[j] = { ...input[i] };
				input[i] = { value: '.', count: dots.count + diff };
				input.splice(j + 1, 0, { value: '.', count: -diff });
				break;
			}

			input[j].checked = true;
		}
	}

	return generateBlockArray(input);
}
