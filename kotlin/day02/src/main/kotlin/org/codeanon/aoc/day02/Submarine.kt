package org.codeanon.aoc.day02

data class Submarine(val depth: Int = 0, val hpos: Int = 0) : Sub {
    override fun addDepth(amount: Int) = copy(depth = depth + amount)
    override fun addPos(amount: Int) = copy(hpos = hpos + amount)
    override fun endState() = hpos * depth
    override fun toString() = "Submarine(depth=$depth, hpos=$hpos)"
}
