import { test, expect } from 'bun:test';
import { solveDay15 } from '.';

test('find correct solution for basic part1', async () => {
	const result = await solveDay15('basic_new', 1);
	expect(result).toBe(2028);
	const result2 = await solveDay15('basic', 1);
	expect(result2).toBe(10092);
});

test('find correct solution for basic part2', async () => {
	const result = await solveDay15('basic1', 2);
	expect(result).toBe(618);
	const result2 = await solveDay15('basic', 2);
	expect(result2).toBe(9021);
});
