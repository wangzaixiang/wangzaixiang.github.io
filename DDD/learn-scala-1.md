---
layout: ddd
prev: learn-scala.html
next: learn-scala-2.html
---
### Phase 1: Scala as an alternative Java

Scala有复杂的语言特性一面，不过，我们可以先把Scala来做一个更好的Java来使用。使用Java的编程思路，但改变为Scala的语法，来替代Java编程。

对于Java程序员来说，转型到使用Scala来进行开发，这是非常重要的一步，可能会不太习惯，但应该可以很快的进行转变，因为单纯语法方面的差异，其实本质上是很小的，只是一个习惯的转换，培养一定的scala语感。

* 使用 object 来替代 Java 的static。在scala中，没有static,将static的方法、变量定义在 object 中。
* 使用 trait 来替代 Java interface。从Scala 2.12开始，trait 和 java interface越来越接近了。default method等最早就是 trait 的行为，现在合并到Java之中。
* 使用 Scala的异常处理模式来替代 try/catch。
* Scala中，在类定义中，直接申明构造参数，而无需申明独立的构造方法。每个类有一个主构造函数，而其它的辅助构造函数是对其的一个二次封装。
* 使用 while 来替代传统的循环。Scala的for是完全不同于Java中的循环的，我们避免使用for来简单替代循环。
* 使用 `Boolean/Byte/Char/Short/Int/Long/Float/Double`等类型来替代 `boolean/byte/char/short/int/long/float/double` 等基础类型。
* 使用 `Array[Byte]` 来替代 `byte[]`
* 在scala中，使用`array(idx)` 来替代java版本的`array[idx]`
* 使用 match 来替代 Java的 switch。
* 熟悉 Scala 的替代变量声明、方法定义的语法。
* Scala中缺少enum, @annotation的申明的支持。如果需要枚举、标注，需要在Java中定义，然后在scala中使用。

大部分的Java语法都可以简单的映射为Scala，这里列出一些需要关注的点：

1. Scala不支持定义enum，如果需要，使用Java定义枚举，然后在Scala中使用他。
2. Scala不支持定义annotation，如果需要，使用Java定于annotation，然后在Scala中使用他。
3. instanceof 与 强制类型转换。
   ```scala
   if( obj.isInstanceOf[String] ) {
      val str = obj.asInstanceOf[String]
   }

   ```
1. 使用`classOf[String]`替代`String.class`
2. break. scala中没有break关键字，虽然可以使用`scala.util.control.Breaks`替代，但尽量避免使用。
3. return的使用。 在scala中尽量避免使用 return 提前从方法中返回。必要的时候采用 if/else 进行处理。
4. scala没有 `?:` 三元操作符，需要用 `if else`替代。
5. 使用 `obj.synchronized { }` 来处理Java同步块。

综上，你不需要太多的学习scala，就可以简单的使用scala的语法 + java的编程模式，来开始尝试编写代码。即使这样，你可以尝试享受scala的优点：

* 使用scala的类型推理机制，声明变量，让代码更显简洁。
* 少使用`;`作为代码的分隔，让代码更加简洁

从我们的实践而言，我们要求Java程序员使用2-3天的时间来切入这个过程，不需要对Scala语言特别的熟练，但关键是“写”，尝试使用Scala来替代Java解决一些基本的编程问题，完成300-1000行代码的编码实践，形成初步的语感。然后在1周的时间内，恢复Java的编程效率，即对同一件任务，如果，使用Java需要1人天的工时，那么，至少也可以达到同样的熟练程度，可以在1人天的工时内，使用Scala完成任务。







