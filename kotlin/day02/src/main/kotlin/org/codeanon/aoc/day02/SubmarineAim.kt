package org.codeanon.aoc.day02

data class SubmarineAim(val depth: Int = 0, val pos: Int = 0, val aim: Int = 0): Sub {
    override fun addDepth(amount: Int) = copy(aim = aim + amount)
    override fun addPos(amount: Int) = copy(pos = pos + amount, depth = depth + amount * aim)
    override fun endState() = depth * pos
    override fun toString() = "SubmarineAim(depth=$depth, pos=$pos, aim=$aim)"
}
