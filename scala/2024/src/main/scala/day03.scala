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
  val matches = allPtn.findAllIn(infile)
  var isActive = true
  var pt2 = 0
  for p <- matches.matchData do
    p.toString match {
      case "do()"    => isActive = true
      case "don't()" => isActive = false
      case _ =>
        if isActive then
          val m = mulPtn.findFirstMatchIn(p.toString)
          m match {
            case Some(m) => pt2 = pt2 + m.group(1).toInt * m.group(2).toInt
            case _       =>
          }
    }
  println(s"Part 2: $pt2")

def parse(input: String): Array[Array[Int]] =
  input.split("\n").map(_.split(" ").map(_.toInt))
