import { test, expect } from "bun:test";
import { solveDay1 } from ".";

test("find correct solution for basic part1", async () => {
	const result = await solveDay1("./day1", "basic", 1);
	expect(result).toBe(11);
});

test("find correct solution for basic part2", async () => {
	const result = await solveDay1("./day1", "basic", 2);
	expect(result).toBe(31);
});
