package day02

val VALID_SIZES = Array(1, 2, 3)

@main def solve(): Unit =
  val infile = io.Source.fromFile("./data/day02.input").mkString
  var reports = parse(infile)

  val isValidReports = reports.filter(isValid).size

  val pt1 = isValidReports
  println(s"Part 1: $pt1")

  val pt2 = 0
  println(s"Part 2: $pt2")

def parse(input: String): Array[Array[Int]] =
  input.split("\n").map(_.split(" ").map(_.toInt))

def isValid(report: Array[Int]): Boolean =
  val diffs = report
    .sliding(2)
    .map(w => w(1) - w(0))
    .toArray

  val allCorrectSize =
    diffs.map(VALID_SIZES contains Math.abs(_)).fold(true)(_ && _)
  val allPositiveDiffs = diffs.map(_ > 0).fold(true)(_ && _)
  val allNegativeDiffs = diffs.map(_ < 0).fold(true)(_ && _)

  allCorrectSize && (allPositiveDiffs || allNegativeDiffs)
