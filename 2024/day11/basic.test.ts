import { test, expect } from 'bun:test';
import { solveDay11 } from '.';

test('find correct solution for basic part1', async () => {
	const result = await solveDay11('basic', 1);
	expect(result).toBe(55312);
});

// test('find correct solution for basic part2', async () => {
// 	const result = await solveDay11('basic', 2);
// 	expect(result).toBe(81);
// });
