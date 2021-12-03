import org.codeanon.aoc.day02.*
import org.codeanon.aoc.platform.Challenge
import org.codeanon.aoc.platform.Runner

class Day02 : Challenge {
    override fun run01(data: String) {
        process(data, Submarine())
    }

    override fun run02(data: String) {
        process(data, SubmarineAim())
    }

    @Suppress("NAME_SHADOWING")
    private fun process(data: String, sub: Sub) {
        val sub = data
            .lineSequence()
            .map { parseCommand(it) }
            .filterNotNull()
            .fold(sub)
            { sub, cmd ->
                when(cmd.dir) {
                    Direction.DOWN -> sub.addDepth(cmd.amount)
                    Direction.UP -> sub.addDepth(-cmd.amount)
                    Direction.FWD -> sub.addPos(cmd.amount)
                }
            }
        println(sub)
        print("End state: ${sub.endState()}")
    }
}

fun main(args: Array<String>) {
    Runner(Day02()).run(args)
}