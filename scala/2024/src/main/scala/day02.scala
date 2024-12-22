package day02

val VALID_SIZES = Array(1, 2, 3)

@main def solve(): Unit =
  val infile = io.Source.fromFile("./data/day02.input").mkString
  var reports = parse(infile)

  val pt1 = reports.count(isValid)
  println(s"Part 1: $pt1")

  val pt2 = reports.count(isValidWithTolerance)
  println(s"Part 2: $pt2")

def parse(input: String): Array[Array[Int]] =
  input.split("\n").map(_.split(" ").map(_.toInt))

def isValidWithTolerance(report: Array[Int]): Boolean =
  (0 to report.size).map(report.patch(_, Nil, 1)).exists(isValid)

def isValid(report: Array[Int]): Boolean =
  val diffs = report.sliding(2).map(w => w(1) - w(0)).toArray

  val allCorrectSize = diffs.forall(VALID_SIZES contains Math.abs(_))
  val allPositiveDiffs = diffs.forall(_ > 0)
  val allNegativeDiffs = diffs.forall(_ < 0)

  allCorrectSize && (allPositiveDiffs || allNegativeDiffs)
