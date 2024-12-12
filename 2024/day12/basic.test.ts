import { test, expect } from 'bun:test';
import { solveDay12 } from '.';

test('find correct solution for basic part1', async () => {
	const result = await solveDay12('basic', 1);
	expect(result).toBe(140);

	const secondResult = await solveDay12('basic_new', 1);
	expect(secondResult).toBe(1930);
});

test('find correct solution for basic part2', async () => {
	const secondResult = await solveDay12('basic', 2);
	expect(secondResult).toBe(80);

	const result = await solveDay12('basic_new', 2);
	expect(result).toBe(1206);

	const other = await solveDay12('basic1', 2);
	expect(other).toBe(236);

	const stuff = await solveDay12('basic2', 2);
	expect(stuff).toBe(368);
});
