package org.codeanon.aoc.day02

interface Sub {
    fun addDepth(amount: Int): Sub
    fun addPos(amount: Int): Sub
    fun endState(): Int
}