fun bomb_maker(size: Int) {

    val n = (0..(size/2)).random()

    //println("n: $n")

    val row = ArrayList<String>()
    val list: MutableList<Int> = (1..size).toMutableList()
    list.shuffle()
    //println("list: $list")

    for (i in 0 until size) {
        if (list[i]<=n) {
            row.add("x")
        }
        else {
            row.add(".")
        }
    }

    val separator = " "
    val string = row.joinToString(separator)
    println(string)

}

fun main(args: Array<String>) {

    println("Enter minefield size.\n")
    val size = (readLine()!!).toInt()

    for (i in 0 until size) {
        bomb_maker(size)
    }
}