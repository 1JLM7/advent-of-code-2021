class Day01: Challenge {
    override fun run01(data: String) {
        val numIncreasing =
            data.split(Regex("\\s+"))
                .filter { it.isNotEmpty() }
                .map { it.toInt() }
                .windowed(2)
                .count {
                    val last = it[0]
                    val current = it[1]
                    current > last
                }
        println("Num. of increasing measurements: $numIncreasing")
    }

    override fun run02(data: String) {
        val numIncreasing =
            data.split(Regex("\\s+"))
                .filter { it.isNotEmpty() }
                .map { it.toInt() }
                .windowed(3)
                .map { it.sum() }
                .windowed(2)
                .count {
                    val last = it[0]
                    val current = it[1]
                    current > last
                }
        println("Num. of increasing measurements: $numIncreasing")
    }
}

fun main(args: Array<String>) {
    Runner(Day01()).run(args)
}