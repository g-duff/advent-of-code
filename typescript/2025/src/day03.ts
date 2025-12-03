import { readFile } from 'node:fs/promises';

const infile = await readFile('data/day03.input', { encoding: 'utf-8' });
const rows = infile
  .trim()
  .split('\n')
  .map((row) => row.split('').map((cell) => parseInt(cell)));

const findBankJoltageTwoBatteries: (
  batts: Array<number>,
  max: number
) => number = ([battHead, ...battsTail], maxJoltage) => {
  if (battsTail.length == 0) return maxJoltage;
  maxJoltage = battsTail.reduce(
    (m, t) => Math.max(m, battHead * 10 + t),
    maxJoltage
  );
  return findBankJoltageTwoBatteries(battsTail, maxJoltage);
};

const findBankJoltage = (
  batts: Array<number>,
  max: number,
  remaining: number
): number => {
  if (remaining == 0) return max;
  const maxBatt = Math.max(...batts.slice(0, batts.length + 1 - remaining));
  const maxBattId = batts.indexOf(maxBatt)!;
  return findBankJoltage(
    batts.slice(maxBattId + 1),
    max * 10 + maxBatt,
    remaining - 1
  );
};

const pt1 = rows
  .map((bank) => findBankJoltageTwoBatteries(bank, 0))
  .reduce((acc, elem) => acc + elem, 0);

const pt2 = rows
  .map((bank) => findBankJoltage(bank, 0, 12))
  .reduce((acc, elem) => acc + elem, 0);

console.log(`Part 1: ${pt1}`);
console.log(`Part 2: ${pt2}`);
