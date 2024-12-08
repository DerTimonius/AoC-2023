import { test, expect } from 'bun:test';
import { solveDay8 } from '.';

test('find correct solution for basic part1', async () => {
	const result = await solveDay8('basic', 1);
	expect(result).toBe(14);
});

test('find correct solution for basic part2', async () => {
	const result = await solveDay8('basic', 2);
	expect(result).toBe(34);
});
