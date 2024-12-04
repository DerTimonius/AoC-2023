import { test, expect } from 'bun:test';
import { solveDay4 } from '.';

test('find correct solution for basic part1', async () => {
	const result = await solveDay4('basic', 1);
	expect(result).toBe(18);
});

test('find correct solution for basic part2', async () => {
	const result = await solveDay4('basic', 2);
	expect(result).toBe(9);
});
