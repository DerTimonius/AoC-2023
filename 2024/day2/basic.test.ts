import { test, expect } from 'bun:test';
import { checkSafety, parseLine, solveDay2 } from '.';

test('correctly parse line', () => {
	expect(parseLine('7 6 4 2 1')).toMatchObject(['7', '6', '4', '2', '1']);
	expect(parseLine('1 2 7 8 9')).toMatchObject(['1', '2', '7', '8', '9']);
});

test('correctly checks safety', () => {
	expect(checkSafety([7, 6, 4, 2, 1])).toBeTrue();
	expect(checkSafety([1, 2, 7, 8, 9])).toBeFalse();
	expect(checkSafety([1, 3, 2, 4, 5])).toBeFalse();
});

test('find correct solution for basic part1', async () => {
	const result = await solveDay2('basic', 1);
	expect(result).toBe(2);
});

test('find correct solution for basic part2', async () => {
	const result = await solveDay2('basic', 2);
	expect(result).toBe(4);
});
