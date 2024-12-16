export async function solveDay16(
	type: 'actual' | 'basic' | 'basic_new' | 'basic1' | 'basic2',
	part: 1 | 2,
): Promise<number> {
	const file = Bun.file(`${import.meta.dir}/${type}.txt`);
	const text = await file.text();
	const grid = text.split('\n').map((row) => row.split(''));

	// console.table(grid);
	return part === 1 ? solvePart1(grid) : solvePart2(grid);
}

type Position = [number, number];
type Direction = 'N' | 'S' | 'E' | 'W';
type NodeId = string;

const DIRECTIONS = {
	N: [-1, 0],
	S: [1, 0],
	E: [0, 1],
	W: [0, -1],
} as const;

class PriorityQueue<T> {
	private items: { item: T; cost: number }[] = [];

	push(item: T, cost: number) {
		let i = 0;
		while (i < this.items.length && this.items[i].cost <= cost) {
			i++;
		}
		this.items.splice(i, 0, { item, cost });
	}

	pop(): T | undefined {
		return this.items.shift()?.item;
	}

	isEmpty(): boolean {
		return this.items.length === 0;
	}
}

function findOptimalPaths(grid: string[][]): {
	minCost: number;
	optimalCells: Set<string>;
} {
	const queue = new PriorityQueue<NodeId>();
	const costs = new Map<NodeId, number>();
	const parents = new Map<NodeId, NodeId[]>();

	let start: Position = [0, 0];
	let end: Position = [0, 0];
	for (let i = 0; i < grid.length; i++) {
		for (let j = 0; j < grid[i].length; j++) {
			if (grid[i][j] === 'S') start = [i, j];
			if (grid[i][j] === 'E') end = [i, j];
		}
	}

	const startNodeId = createNodeId(start, 'E');
	queue.push(startNodeId, 0);
	costs.set(startNodeId, 0);

	while (!queue.isEmpty()) {
		const currentId = queue.pop();
		if (!currentId) break;

		const {
			pos: [row, col],
			dir: currentDir,
		} = parseNodeId(currentId);
		const currentCost = costs.get(currentId) ?? Number.POSITIVE_INFINITY;

		for (const [nextDir, [dr, dc]] of Object.entries(DIRECTIONS)) {
			const newRow = row + dr;
			const newCol = col + dc;

			if (
				newRow < 0 ||
				newRow >= grid.length ||
				newCol < 0 ||
				newCol >= grid[0].length ||
				grid[newRow][newCol] === '#'
			) {
				continue;
			}

			const moveCost = nextDir === currentDir ? 1 : 1001;
			const newCost = currentCost + moveCost;
			const nextNodeId = createNodeId([newRow, newCol], nextDir as Direction);

			// biome-ignore lint/style/noNonNullAssertion: existence ensured
			if (!costs.has(nextNodeId) || newCost < costs.get(nextNodeId)!) {
				costs.set(nextNodeId, newCost);
				queue.push(nextNodeId, newCost);
				parents.set(nextNodeId, [currentId]);
			} else if (newCost === costs.get(nextNodeId)) {
				const currentPaths = parents.get(nextNodeId) ?? [];
				parents.set(nextNodeId, [...currentPaths, currentId]);
			}
		}
	}

	const endCosts = Object.keys(DIRECTIONS).map(
		(dir) =>
			costs.get(createNodeId(end, dir as Direction)) ??
			Number.POSITIVE_INFINITY,
	);
	const minCost = Math.min(...endCosts);

	const optimalCells = new Set<string>();

	function reconstructPaths(nodeId: NodeId, visited: Set<string>) {
		const {
			pos: [row, col],
		} = parseNodeId(nodeId);
		const posKey = `${row},${col}`;
		optimalCells.add(posKey);

		const parentNodes = parents.get(nodeId) ?? [];
		for (const parentId of parentNodes) {
			if (!visited.has(parentId)) {
				visited.add(parentId);
				reconstructPaths(parentId, visited);
			}
		}
	}

	// biome-ignore lint/complexity/noForEach: acceptable
	Object.keys(DIRECTIONS).forEach((dir) => {
		const endNodeId = createNodeId(end, dir as Direction);
		if (costs.get(endNodeId) === minCost) {
			reconstructPaths(endNodeId, new Set<string>());
		}
	});

	return { minCost, optimalCells };
}

function _visualizePath(grid: string[][], optimalCells: Set<string>): string {
	return grid
		.map((row, i) =>
			row
				.map((cell, j) => (optimalCells.has(`${i},${j}`) ? 'O' : cell))
				.join(''),
		)
		.join('\n');
}

function createNodeId(pos: Position, dir: Direction): NodeId {
	return `${pos[0]},${pos[1]},${dir}`;
}

function parseNodeId(id: NodeId): { pos: Position; dir: Direction } {
	const [row, col, dir] = id.split(',');
	return {
		pos: [Number.parseInt(row), Number.parseInt(col)],
		dir: dir as Direction,
	};
}

function solvePart1(grid: string[][]): number {
	const result = findOptimalPaths(grid);

	return result.minCost;
}

function solvePart2(grid: string[][]): number {
	const result = findOptimalPaths(grid);

	return result.optimalCells.size;
}
