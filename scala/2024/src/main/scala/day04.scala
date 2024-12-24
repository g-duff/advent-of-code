package day04

@main def solve(): Unit =
  val infile = parse(io.Source.fromFile("./data/day04.input").mkString)

  val R = infile.size
  val C = infile(0).size

  val findPattern = (rOffset, cOffset, pattern) =>
    findPatternIn(infile, R, C, rOffset, cOffset, pattern)

  val pt1 = List(
    (0, 3, List((0, 0, 'X'), (0, 1, 'M'), (0, 2, 'A'), (0, 3, 'S'))),
    (0, 3, List((0, 0, 'S'), (0, 1, 'A'), (0, 2, 'M'), (0, 3, 'X'))),
    (3, 0, List((0, 0, 'X'), (1, 0, 'M'), (2, 0, 'A'), (3, 0, 'S'))),
    (3, 0, List((0, 0, 'S'), (1, 0, 'A'), (2, 0, 'M'), (3, 0, 'X'))),
    (3, 3, List((0, 0, 'X'), (1, 1, 'M'), (2, 2, 'A'), (3, 3, 'S'))),
    (3, 3, List((0, 0, 'S'), (1, 1, 'A'), (2, 2, 'M'), (3, 3, 'X'))),
    (3, 3, List((0, 3, 'X'), (1, 2, 'M'), (2, 1, 'A'), (3, 0, 'S'))),
    (3, 3, List((0, 3, 'S'), (1, 2, 'A'), (2, 1, 'M'), (3, 0, 'X')))
  ).map(findPattern.tupled).sum
  println(s"Part1: $pt1")

  val pt2 = List(
    (2, 2, List((1, 1, 'A'), (0, 0, 'M'), (0, 2, 'M'), (2, 2, 'S'), (2, 0, 'S'))),
    (2, 2, List((1, 1, 'A'), (0, 0, 'S'), (0, 2, 'M'), (2, 2, 'M'), (2, 0, 'S'))),
    (2, 2, List((1, 1, 'A'), (0, 0, 'S'), (0, 2, 'S'), (2, 2, 'M'), (2, 0, 'M'))),
    (2, 2, List((1, 1, 'A'), (0, 0, 'M'), (0, 2, 'S'), (2, 2, 'S'), (2, 0, 'M')))
  ).map(findPattern.tupled).sum
  println(s"Part2: $pt2")

def parse(input: String): Array[Array[Char]] =
  input.split("\n").map(_.toCharArray())

def findPatternIn(
    infile: Array[Array[Char]],
    R: Int,
    C: Int,
    rOffset: Int,
    cOffset: Int,
    pattern: List[(Int, Int, Char)]
) =
  val coords =
    (0 to R - rOffset - 1).flatMap(r => (0 to C - cOffset - 1).map(c => (r, c)))

  coords.count { (r, c) =>
    pattern.forall { (charR, charC, char) =>
      infile(r + charR)(c + charC) == char
    }
  }
