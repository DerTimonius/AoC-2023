import { test, expect } from 'bun:test';
import { parseSequences, solveDay5 } from '.';

test('correctly parses sequence', () => {
	expect(
		parseSequences(
			`75,47,61,53,29
75,29,13
97,13,75,29,47`.split('\n'),
		),
	).toMatchObject([
		[75, 47, 61, 53, 29],
		[75, 29, 13],
		[97, 13, 75, 29, 47],
	]);
});

test('find correct solution for basic part1', async () => {
	const result = await solveDay5('basic', 1);
	expect(result).toBe(143);
});

test('find correct solution for basic part2', async () => {
	const result = await solveDay5('basic', 2);
	expect(result).toBe(123);
});
