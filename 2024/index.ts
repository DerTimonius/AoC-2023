import { solveDay1 } from './day1';

async function solve(
	cb: (type: 'basic' | 'actual', part: 1 | 2) => Promise<number>,
) {
	const result = await cb('actual', 2);
	console.log('The result is: ', result);
}

solve(solveDay1);
