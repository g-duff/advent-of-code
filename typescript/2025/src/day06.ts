import { readFile } from 'node:fs/promises';

const transpose = (matrix: Array<Array<any>>) =>
  matrix[0].map((_, i) => matrix.map((row) => row[i]));

const add = (a: number, b: number) => a + b;
const mul = (a: number, b: number) => a * b;

const infile = await readFile('data/day06.input', { encoding: 'utf-8' });

const grid = infile
  .split('\n')
  .map((row) => row.split(/\s/).filter((cell) => cell != ''))
  .filter((row) => row.length != 0);

const operations = grid.pop()!;

const R = grid.length;
const C = grid[0].length;

////////////
// Part 1 //
////////////

var pt1 = 0;
for (var c = 0; c < C; c++) {
  const op: (a: number, b: number) => number = operations[c] == '+' ? add : mul;

  var colTotal = operations[c] == '+' ? 0 : 1;
  for (var r = 0; r < R; r++) {
    colTotal = op(colTotal, parseInt(grid[r][c]));
  }
  pt1 += colTotal;
}

console.log(pt1);

////////////
// Part 2 //
////////////

const gridPt2 = infile.split('\n').map((row) => row.split(''));
gridPt2.pop();
const gridT = transpose(gridPt2).map((row) => {
  const last = row.pop();
  return [last, ...row];
});

console.log(gridT);

var pt2 = 0;
var op = add;
var problemTotal = 0;
for (const [first, ...rest] of gridT) {
  if (first == '*') {
    pt2 += problemTotal;
    op = mul;
    problemTotal = 1;
  }
  if (first == '+') {
    pt2 += problemTotal;
    op = add;
    problemTotal = 0;
  }

  const n = parseInt(rest.join(''));
  if (isNaN(n)) continue;
  else problemTotal = op(problemTotal, n);
}
pt2 += problemTotal;

console.log(pt2);
