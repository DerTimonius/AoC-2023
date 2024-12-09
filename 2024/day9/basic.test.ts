import { test, expect } from 'bun:test';
import { solveDay9 } from '.';

test('find correct solution for basic part1', async () => {
	const result = await solveDay9('basic', 1);
	expect(result).toBe(1928);
});

test('find correct solution for basic part2', async () => {
	const result = await solveDay9('basic', 2);
	expect(result).toBe(2858);
});
