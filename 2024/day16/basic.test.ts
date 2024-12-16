import { test, expect } from 'bun:test';
import { solveDay16 } from '.';

test('find correct solution for basic part1', async () => {
	const result = await solveDay16('basic', 1);
	expect(result).toBe(7036);
	const result2 = await solveDay16('basic_new', 1);
	expect(result2).toBe(11048);
	const result3 = await solveDay16('basic1', 1);
	expect(result3).toBe(21148);
	const result4 = await solveDay16('basic2', 1);
	expect(result4).toBe(5078);
});

test('find correct solution for basic part2', async () => {
	const result2 = await solveDay16('basic', 2);
	const result = await solveDay16('basic_new', 2);
	const result3 = await solveDay16('basic1', 2);
	expect(result2).toBe(45);
	expect(result3).toBe(149);
	expect(result).toBe(64);
});
