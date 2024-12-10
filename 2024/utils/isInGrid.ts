export function isInGrid<T>(grid: T[][], row: number, col: number): boolean {
	return row >= 0 && col >= 0 && row < grid.length && col < grid[row].length;
}
