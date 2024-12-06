import { test, expect, describe } from 'bun:test';
import { findStart, isInGrid, solveDay6 } from '.';

describe('grid tests', async () => {
	const file = Bun.file(`${import.meta.dir}/basic.txt`);
	const text = await file.text();

	const grid = text
		.trim()
		.split('\n')
		.map((line) => line.split('').map((char) => ({ char, visited: false })));

	test('finds start', () => {
		expect(findStart(grid)).toMatchObject([6, 4]);
	});

	test('detects out of bounds', () => {
		expect(isInGrid(grid, 6, 4)).toBeTrue();
		expect(isInGrid(grid, 9, 14)).toBeFalse();
	});
});
test('find correct solution for basic part1', async () => {
	const result = await solveDay6('basic', 1);
	expect(result).toBe(41);
});

test('find correct solution for basic part2', async () => {
	const result = await solveDay6('basic', 2);
	expect(result).toBe(6);
});
