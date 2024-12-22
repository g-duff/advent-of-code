package day01

import scala.io

@main def solve(): Unit =
  val infile = io.Source.fromFile("./data/day01.input").mkString
  var (col0, col1) = parse(infile)

  col0 = col0.sortWith(_ < _)
  col1 = col1.sortWith(_ < _)

  val pt1 = (col0 zip col1).map((c0, c1) => Math.abs(c1 - c0)).sum
  println(s"Part 1: $pt1")

  // Assume we'll use evaluated counts more than once
  val counts = col0.groupMap(identity)(identity).mapValues(_.size)
  val pt2 = col1.map(n => n * counts.getOrElse(n, 0)).sum
  println(s"Part 2: $pt2")

def parse(input: String): (Array[Int], Array[Int]) =
  var lines = input.split("\n").map(_.split("   "))
  (lines.map(_(0).toInt), lines.map(_(1).toInt))
