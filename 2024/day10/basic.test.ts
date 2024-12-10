import { test, expect } from 'bun:test';
import { solveDay10 } from '.';

test('find correct solution for basic part1', async () => {
	const result = await solveDay10('basic', 1);
	expect(result).toBe(36);
});

test('find correct solution for basic part2', async () => {
	const result = await solveDay10('basic', 2);
	expect(result).toBe(81);
});
