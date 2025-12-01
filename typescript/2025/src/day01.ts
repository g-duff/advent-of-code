import { readFile } from 'node:fs/promises';

const infile = await readFile('data/day01.input', { encoding: 'utf-8' });
const rows = infile.trim().split('\n');

const { pt1, pt2 } = rows.reduce(
  ({ pos, pt1, pt2 }, row) => {
    const [dir, _] = row;
    const count = parseInt(row.slice(1));

    const twist = dir == 'R' ? (p: number) => p + 1 : (p: number) => p - 1;

    for (var n = 0; n < count; n++) {
      pos = twist(pos);
      if (pos % 100 == 0) pt2++;
    }

    if (pos % 100 == 0) pt1++;

    return { pos, pt1, pt2 };
  },
  { pos: 50, pt1: 0, pt2: 0 }
);

console.log(`Part 1: ${pt1}`);
console.log(`Part 2: ${pt2}`);
