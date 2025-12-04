import { readFile } from 'node:fs/promises';

const directions = [
  [-1, -1],
  [-1, 0],
  [-1, 1],
  [0, -1],
  [0, 1],
  [1, -1],
  [1, 0],
  [1, +1],
];

const getCell = (grid: string[][], r: number, c: number): string => {
  if (r < 0 || c < 0) return '.';
  const row = grid.at(r);
  if (!row) return '.';
  const cell = row.at(c);
  if (!cell) return '.';
  return cell;
};

const accessibleRolls = (grid: string[][], coords: [number, number][]) =>
  coords
    .filter(([r, c]) => grid[r][c] == '@')
    .filter(
      ([r, c]) =>
        directions
          .map(([dr, dc]) => getCell(rows, r + dr, c + dc))
          .reduce((acc, c) => (c == '@' ? acc + 1 : acc), 0) < 4
    );

const infile = await readFile('data/day04.input', { encoding: 'utf-8' });
const rows = infile
  .trim()
  .split('\n')
  .map((row) => row.split(''));

const R = rows.length;
const C = rows[0].length;
const rowIdxs = Array(R).fill(0);
const colIdxs = Array(C).fill(0);
const coords = rowIdxs
  .map((_, r) => colIdxs.map((_, c) => [r, c]))
  .flat() as Array<[number, number]>;

const pt1 = accessibleRolls(rows, coords).length;
console.log(pt1);

var pt2 = 0;
var removed = -1;

while (removed != 0) {
  const toRemove = accessibleRolls(rows, coords);
  removed = toRemove.length;
  pt2 += toRemove.length;
  for (const [rr, cc] of toRemove) {
    rows[rr][cc] = '.';
  }
}

console.log(pt2);
