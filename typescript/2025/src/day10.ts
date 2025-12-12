import { readFile } from 'node:fs/promises';

const infile = await readFile('data/day10.input', { encoding: 'utf-8' });
const input = infile
  .trim()
  .split('\n')
  .map((row) => {
    const splitRow = row.split(' ');
    splitRow.pop();
    const [unparsedTarget, ...unparsedButtons] = splitRow;
    const target = unparsedTarget
      .replace('[', '')
      .replace(']', '')
      .split('')
      .map((c) => c == '#');
    const buttons = unparsedButtons.map((b) =>
      b
        .replace('(', '')
        .replace(')', '')
        .split(',')
        .map((x) => parseInt(x))
    );
    return {
      target,
      buttons,
    };
  });

const switchOn = (
  target: boolean[],
  indicatorLights: boolean[],
  buttons: number[][],
  pressed: boolean[],
  memo: Map<string, number>
): number => {
  if (indicatorLights.every((b, idx) => b == target[idx]))
    return pressed.reduce((acc, p) => (acc += p ? 1 : 0), 0);
  if (pressed.every((p) => p == true)) return Infinity;

  return Math.min(
    ...buttons.map((b, idx) => {
      const newPressed = pressed.slice();

      if (newPressed[idx]) return Infinity;

      const newIndicatorLights = b.reduce((acc, bb) => {
        acc[bb] = !acc[bb];
        return acc;
      }, indicatorLights.slice());

      newPressed[idx] = true;

      const key = JSON.stringify(newPressed);
      if (memo.has(key)) return memo.get(key)!;

      const val = switchOn(
        target,
        newIndicatorLights,
        buttons,
        newPressed,
        memo
      );
      memo.set(key, val);
      return val;
    })
  );
};

var pt1 = 0;
for (const { buttons, target } of input) {
  const memo = new Map();
  const indicatorLights = new Array(target.length).fill(false);
  const pressed = new Array(buttons.length).fill(false);
  const buttonPresses = switchOn(
    target,
    indicatorLights,
    buttons,
    pressed,
    memo
  );
  pt1 += buttonPresses;
}

console.log(pt1);
