interface Rules {
	after: number[];
	before: number[];
}

type RuleMap = Map<number, Rules>;

export async function solveDay5(
	type: 'actual' | 'basic' | 'basic_new',
	part: 1 | 2,
): Promise<number> {
	const file = Bun.file(`${import.meta.dir}/${type}.txt`);
	const text = await file.text();

	const blocks = text.trim().split('\n\n');
	const rules = parseRules(blocks[0].split('\n'));
	const sequences = parseSequences(blocks[1].split('\n'));

	return part === 1
		? solvePart1(rules, sequences)
		: solvePart2(rules, sequences);
}

function solvePart1(ruleMap: RuleMap, sequences: number[][]): number {
	const sum = sequences.reduce((acc, sequence) => {
		if (checkSequence(sequence, ruleMap)) {
			return acc + sequence[Math.floor(sequence.length / 2)];
		}

		return acc;
	}, 0);

	return sum;
}

function solvePart2(ruleMap: RuleMap, sequences: number[][]): number {
	const unordered = sequences.filter((s) => !checkSequence(s, ruleMap));
	const sum = unordered.reduce((acc, sequence) => {
		const ordered = reorderSequence(sequence, ruleMap);

		return acc + ordered[Math.floor(sequence.length / 2)];
	}, 0);

	return sum;
}

function checkSequence(sequence: number[], ruleMap: RuleMap): boolean {
	for (let i = 0; i < sequence.length; i++) {
		for (let j = i + 1; j < sequence.length; j++) {
			const firstRule = ruleMap.get(sequence[i]);
			const secondRule = ruleMap.get(sequence[j]);
			if (!firstRule || !secondRule) {
				continue;
			}

			if (
				!firstRule.after.includes(sequence[j]) ||
				!secondRule.before.includes(sequence[i])
			)
				return false;
		}
	}

	return true;
}

function reorderSequence(sequence: number[], ruleMap: RuleMap): number[] {
	const ordered: number[] = [];
	for (const num of sequence) {
		if (!ordered.length) {
			ordered.push(num);
			continue;
		}

		const rule = ruleMap.get(num);
		if (!rule) {
			ordered.push(num);
			continue;
		}

		let insertIndex = ordered.length;
		for (let i = 0; i < ordered.length; i++) {
			const mustBeBefore = rule.before.includes(ordered[i]);
			const mustBeAfter = rule.after.includes(ordered[i]);

			if (mustBeBefore) {
				insertIndex = Math.min(insertIndex, i);
			}

			if (mustBeAfter) {
				insertIndex = Math.max(insertIndex, i + 1);
			}
		}

		ordered.splice(insertIndex, 0, num);
	}
	return ordered;
}

function parseRules(lines: string[]): RuleMap {
	const rules = new Map() as RuleMap;

	for (const line of lines) {
		const [before, after] = line.match(/\d+/g)?.map((x) => Number(x)) ?? [];
		if (!before || !after) {
			throw new Error('incorrect rule length');
		}

		if (rules.has(before)) {
			// biome-ignore lint/style/noNonNullAssertion: checked existence
			const existing = rules.get(before)!;
			const newAfter = [...new Set([...existing.after, after])];
			rules.set(before, { before: existing.before, after: newAfter });
		} else {
			rules.set(before, { before: [], after: [after] });
		}

		if (rules.has(after)) {
			// biome-ignore lint/style/noNonNullAssertion: checked existence
			const existing = rules.get(after)!;
			const newBefore = [...new Set([...existing.before, before])];
			rules.set(after, { before: newBefore, after: existing.after });
		} else {
			rules.set(after, { before: [before], after: [] });
		}
	}
	return rules;
}

export function parseSequences(lines: string[]): number[][] {
	return lines.map((line) => line.match(/\d+/g)?.map((x) => Number(x)) ?? []);
}
