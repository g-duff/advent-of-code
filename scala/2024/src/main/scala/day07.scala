package day07

@main def solve(): Unit =
  val equations = parse(io.Source.fromFile("./data/day07.input").mkString)

  val pt1 = equations
    .filter((equation) => check(false, equation.head, equation.tail, 0))
    .map(_.head)
    .sum
  println(s"Part 1: $pt1")

  val pt2 = equations
    .filter((equation) => check(true, equation.head, equation.tail, 0))
    .map(_.head)
    .sum
  println(s"Part 2: $pt2")

def parse(input: String): Array[Array[Long]] =
  input.split("\n").map(line => line.split(" |: ").map(_.toLong))

def check(
    pt2: Boolean,
    target: Long,
    rest: Array[Long],
    total: Long
): Boolean =
  if total > target then return false

  (rest.size == 0, total == target) match {
    case (true, true)  => true
    case (true, false) => false
    case _ =>
      check(pt2, target, rest.tail, total + rest.head)
      || check(pt2, target, rest.tail, total * rest.head)
      || (pt2 && check(
        pt2,
        target,
        rest.tail,
        (total.toString + rest.head.toString).toLong
      ))
  }
