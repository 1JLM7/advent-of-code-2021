package org.codeanon.aoc.day02

enum class Direction {
    FWD,
    UP,
    DOWN,
}

data class Command(val dir: Direction, val amount: Int) {}

private val lineRegex = Regex("(up|down|forward)\\s+(\\d+)")

fun parseCommand(line: CharSequence): Command? {
    val match = lineRegex.find(line)
    return if (match != null) {
        val dir = when (match.groupValues[1]) {
            "up" -> Direction.UP
            "down" -> Direction.DOWN
            "forward" -> Direction.FWD
            else -> throw IllegalStateException("Unreachable branch of ${match.groupValues[0]}")
        }
        val amount = match.groupValues[2].toInt()
        Command(dir, amount)
    } else null
}
