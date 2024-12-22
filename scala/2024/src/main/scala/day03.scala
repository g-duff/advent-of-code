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

def parse(input: String): Array[Array[Int]] =
  input.split("\n").map(_.split(" ").map(_.toInt))
