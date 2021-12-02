import kotlinx.cli.ArgParser
import kotlinx.cli.ArgType
import kotlinx.cli.default
import kotlinx.coroutines.*
import java.io.File

enum class ChallengeStage {
    Stage1,
    Stage2,
}

class Runner<C : Challenge>(private val challenge: C) {
    fun run(args: Array<String>) = runBlocking<Unit> {
        val parser = ArgParser("Advent of Code 2021")
        val data by parser.argument(ArgType.String, fullName = "Data", description = "Data file provided by the challenge")
        val mode by parser.option(ArgType.Choice<ChallengeStage>(), shortName = "c", fullName = "challenge").default(ChallengeStage.Stage1)

        parser.parse(args)

        launch {
            val file = loadFileAsync(data)
            when(mode) {
                ChallengeStage.Stage1 -> challenge.run01(file.await())
                ChallengeStage.Stage2 -> challenge.run02(file.await())
            }
        }.join()
    }
    private val fileScope = CoroutineScope(Dispatchers.IO)
    private fun loadFileAsync(filename: String) = fileScope.async {
        File(filename).readText()
    }
}