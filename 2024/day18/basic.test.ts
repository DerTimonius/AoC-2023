import { test, expect } from 'bun:test';
import { solveDay18 } from '.';

test('find correct solution for basic part1', async () => {
	const result = await solveDay18('basic', 1);
	expect(result).toBe(22);
});

// test('find correct solution for basic part2', async () => {
// 	const result = await solveDay18('basic_new', 2);
// 	expect(result).toBe(117440);
// });
