package day05

@main def solve(): Unit =
  val infile = io.Source.fromFile("./data/day05.input").mkString
  val (pageOrderingRules, pages) = parse(infile)

  val pt1 = pages
    .filter(page =>
      indexWhereBreaksRule(pageOrderingRules, page) match {
        case Some(_) => false
        case None    => true
      }
    )
    .map(page => page(page.size / 2))
    .sum
  println(s"Part 1: $pt1")

def parse(infile: String): (Map[Int, Array[Int]], Array[Array[Int]]) =
  val bits = infile.split("\n\n")

  val rulesMap = bits(0)
    .split("\n")
    .foldLeft(Map[Int, Array[Int]]())((acc, line) =>
      val nums = line.split('|')
      val k = nums(0).toInt
      val v = acc.get(k) match {
        case Some(l) => l :+ nums(1).toInt
        case None    => Array(nums(1).toInt)
      }
      acc.updated(k, v)
    )

  val pages = bits(1).split('\n').map(page => page.split(',').map(_.toInt))
  (rulesMap, pages)

def indexWhereBreaksRule(
    rules: Map[Int, Array[Int]],
    page: Array[Int]
): Option[(Int, Int)] =
  page.zipWithIndex.find { (p, i) =>
    rules.get(p) match {
      case Some(rule) => page.take(i).exists(rule.contains(_))
      case None       => false
    }
  }
