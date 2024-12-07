import { test, expect } from 'bun:test';
import { parseCalibrations, solveDay7, testCalibration } from '.';

test('correctly parses calibrations', () => {
	const result = parseCalibrations(['190: 10 19', '3267: 81 40 27']);
	expect(result).toMatchObject([
		{ result: 190, nums: [10, 19] },
		{ result: 3267, nums: [81, 40, 27] },
	]);
});

test('correctly checks calibrations', () => {
	const operations = [
		(a: number, b: number) => a + b,
		(a: number, b: number) => a * b,
	];
	expect(
		testCalibration({ result: 190, nums: [10, 19] }, operations),
	).toBeTrue();
	expect(
		testCalibration({ result: 3267, nums: [81, 40, 27] }, operations),
	).toBeTrue();
	expect(
		testCalibration({ result: 192, nums: [17, 8, 14] }, operations),
	).toBeFalse();
});

test('find correct solution for basic part1', async () => {
	const result = await solveDay7('basic', 1);
	expect(result).toBe(3749);
});

test('find correct solution for basic part2', async () => {
	const result = await solveDay7('basic', 2);
	expect(result).toBe(11387);
});
