import { test, expect } from 'bun:test';
import { parseLine, solveDay1 } from '.';

test('correctly parse line', () => {
	expect(parseLine('3   4')).toMatchObject(['3', '4']);
	expect(parseLine('5   2')).toMatchObject(['5', '2']);
});

test('find correct solution for basic part1', async () => {
	const result = await solveDay1('basic', 1);
	expect(result).toBe(11);
});

test('find correct solution for basic part2', async () => {
	const result = await solveDay1('basic', 2);
	expect(result).toBe(31);
});
