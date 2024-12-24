package day04

import scala.util.matching.Regex

@main def solve(): Unit =
  val infile = parse(io.Source.fromFile("./data/day04.input").mkString)

  val R = infile.size
  val C = infile(0).size

  val kernels = List(
    (0, 3, List((0, 0, 'X'), (0, 1, 'M'), (0, 2, 'A'), (0, 3, 'S'))),
    (0, 3, List((0, 0, 'S'), (0, 1, 'A'), (0, 2, 'M'), (0, 3, 'X'))),
    (3, 0, List((0, 0, 'X'), (1, 0, 'M'), (2, 0, 'A'), (3, 0, 'S'))),
    (3, 0, List((0, 0, 'S'), (1, 0, 'A'), (2, 0, 'M'), (3, 0, 'X'))),
    (3, 3, List((0, 0, 'X'), (1, 1, 'M'), (2, 2, 'A'), (3, 3, 'S'))),
    (3, 3, List((0, 0, 'S'), (1, 1, 'A'), (2, 2, 'M'), (3, 3, 'X'))),
    (3, 3, List((0, 3, 'X'), (1, 2, 'M'), (2, 1, 'A'), (3, 0, 'S'))),
    (3, 3, List((0, 3, 'S'), (1, 2, 'A'), (2, 1, 'M'), (3, 0, 'X')))
  )

  val pt1 = kernels
    .map(k => {
      val (rOff, cOff, things) = k
      var count = 0
      for r <- (0 to R - rOff - 1) do
        for c <- (0 to C - cOff - 1) do
          val y = things.forall((x) => {
            var (rr, cc, l) = x
            infile(rr + r)(cc + c) == l
          })
          if y then count += 1
      count
    })
    .sum
  println(s"Part1: $pt1")

def parse(input: String): Array[Array[Char]] =
  input.split("\n").map(_.toCharArray())
