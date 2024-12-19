import { test, expect } from 'bun:test';
import { solveDay19 } from '.';

test('find correct solution for basic part1', async () => {
	const result = await solveDay19('basic', 1);
	expect(result).toBe(6);
});

test('find correct solution for basic part2', async () => {
	const result = await solveDay19('basic', 2);
	expect(result).toBe(16);
});
