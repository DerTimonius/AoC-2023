import { test, expect } from 'bun:test';
import { checkSafety, parseLine, parseMult, solveDay3 } from '.';

test('correctly parse line', () => {
	expect(
		parseLine(
			'xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))',
		),
	).toMatchObject(['mul(2,4)', 'mul(5,5)', 'mul(11,8)', 'mul(8,5)']);
	expect(
		parseLine(
			"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5)",
		),
	).toMatchObject(['mul(2,4)', 'mul(8,5)', 'mul(8,5)']);
});

test('correctly parse mult', () => {
	expect(parseMult('mul(2,4)')).toMatchObject(['2', '4']);
	expect(parseMult('mul(5,5)')).toMatchObject(['5', '5']);
});

test('find correct solution for basic part1', async () => {
	const result = await solveDay3('basic', 1);
	expect(result).toBe(161);
});

test('find correct solution for basic part2', async () => {
	const result = await solveDay3('basic_new', 2);
	expect(result).toBe(48);
});
