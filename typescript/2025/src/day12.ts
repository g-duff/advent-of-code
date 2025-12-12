import { readFile } from 'node:fs/promises';

const infile = await readFile('data/day12.input', { encoding: 'utf-8' });
const rows = infile.trim().split('\n\n');

const regions = rows[rows.length - 1].split('\n').map((row) => {
  const [aarea, ...ccounts] = row.split(' ');
  const area = aarea
    .replace(':', '')
    .split('x')
    .map((x) => parseInt(x));
  const quantities = ccounts.map((x) => parseInt(x));
  return { area, quantities };
});

var pt1 = 0;
for (const { area, quantities } of regions) {
  console.log(area, quantities);
  const regionArea = area[0] * area[1];
  const presentsArea = quantities.reduce((acc, c) => acc + c, 0) * 9;
  pt1 += regionArea >= presentsArea ? 1 : 0;
}

console.log(pt1);
