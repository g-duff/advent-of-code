package day03

import scala.util.matching.Regex

@main def solve(): Unit =
  val infile = io.Source.fromFile("./data/day03.input").mkString

  val mulPtn = """mul\((\d+),(\d+)\)""".r
  val pt1 = mulPtn
    .findAllIn(infile)
    .matchData
    .map(m => m.group(1).toInt * m.group(2).toInt)
    .sum
  println(s"Part 1: $pt1")

  val allPtn = """mul\((\d+),(\d+)\)|don't\(\)|do\(\)""".r
  val pt2 = allPtn
    .findAllIn(infile)
    .foldLeft(0, true) {
      case ((tot, _), "do()")          => (tot, true)
      case ((tot, _), "don't()")       => (tot, false)
      case ((tot, true), mulPtn(a, b)) => (tot + a.toInt * b.toInt, true)
      case (acc, _)                    => acc
    }(0)
  println(s"Part 2: $pt2")
