---
layout: ddd
prev: learn-scala-1.html
next: learn-scala-3.html
---

### Phase 2: Scala as better Java

在第一个阶段，基本掌握了基本的scala语法，能够用scala来替代java来完成编程，虽然代码的风格还是完全Java化的，但至少可以达到或者接近你之前的编码效率。这其实是一个不难的任务，是所有Java程序员首先要达到的程度。

然后，我们将进一步的熟悉Scala的一些进阶特性，开启我们学习Scala的第二阶段：作为更好的Java语言来使用。

1. 使用 expression 来替代 statement。
   
   在Java中，我们更多使用 statement，所谓 statement，就是产生副作用的一个语句、操作。
   - if/else 是分支语句，不同的分支中可以做不同的时期。
   - for/while 是循环语句，构造一个循环块。
   - switch 分之语句
   - 赋值操作， 这里的副作用就是修改了变量的值。
   - break, return, continue 是改变流程的语句。

   在Scala中，更为重要的是expression，所谓 expression，就是一个表达式，会返回一个值，其重心并不是产生副作用。
    
   if/else可以作为表达式：
   ```scala
   val showFullName = true
   var name: String
   
   // 将 if/else作为 statement使用
   if(showFullName) name = firstName + " " + lastName
   else name = firstName
   
   // 将if/else作为expression使用
   val name = 
      if(showFullName) firstName + " " + lastName
      else firstName
   ```
   
   for可以作为表达式（而不是循环语句）：
   ```scala   
   // 将for作为statement使用
   val origin = 0 to 10
   var squres = new Array[Int](10)
   for(i <- origin){
      squares(i) = origin(i) * origin(i)
   } 
   
   // 将for作为表达式使用
   val squares = for(i <- origin) yield origin(i) * origin(i)
   ```
   
   match 可以作为表达式：
   ```scala
   // 将match作为statement使用
   val month = 2
   var days: Int
   month match {
   case 1 => days = 31
   case 2 => days = 28
   case 3 => days = 31
   ......
   }
   
   // 将match作为expression使用
   val days = month match {
   case 1 => 31
   case 2 => 28
   ```
   try/catch 可以作为 expression 使用：
   ```scala
   // 将try/catch 作为statment使用
   var number = -1
   try {
     number = Integer.parseNumber(input)
   }
   catch {
     case ex: NumberFormatException =>
       number = -1
   }
   
   // 将try/catch作为 expr 使用
   val number = try { Integer.parseNumber(input) }
   catch { case ex: NumberFormatException => -1 }
   ```
   
   其实，上面的几个例子都有一个共同点，使用expression替 statement后，都消除了var，而使用val替代。这其实是非常关键的一点。记住，**在Scala中，var是邪恶的，Scala不需要var**。 
   
2. 学习 Collection API，掌握丰富的集合操作。
   几乎所有的介绍函数式编程的语言，都会包括这样的例子：
   ```scala
   val text = "a long string contains many words"
   val words = text.split("\\s+")
     .map(_.toLowerCase)
     .filter(NON_WORDS.contains(_)==false)
     .groupBy(identity)
     .foreach { case (word, array) =>
        println("$word count:${array.size}")
     }
    ```
  如果使用Java的模式来完成同样的功能，不仅代码量会膨胀很多倍，而且，可读性也会成为一个很大的问题。
  而这也是函数式编程最为擅长的，在上面的例子中，map, filter, groupBy, foreach都是高阶函数，通过这些高阶函数的组合，数据以管道的方式从一个结算传递给另外一个计算，简洁而优美。
  * map operation
     * xs map f
     * xs flatMap f
     * xs collect f
  * iteration
    * xs foreach f
  * filter & subcollections
    * xs.head
    * xs.tail
    * xs.init
    * xs take n
    * xs drop n
    * xs takeWhile p
    * xs dropWhile p
    * xs filter p
    * xs filterNot p
 * test
    * xs forall p
    * xs exists p
    * xs count p
 * Fold Operation
    * xs foldLeft(z)(op)
    * xs foldRight(z)(op)
    * xs reduceLeft op
    * xs reduceRight op
    * xs.sum
    * xs.product
    * xs.min
    * xs.max
 
 scala的进阶之旅，熟练掌握上述的集合操作，并且能够在编程时，熟练的应用，消除一下传统的Java编程方式：
 * 不再需要使用 for 或者 while 等传统的Java模式来处理集合。
 * 不在需要使用 var 乃至 mutable collection来处理集合。
 
 你会感觉到scala的强大、美妙，而且这样编写的代码，不仅简洁、易于阅读，而且，程序的Bug会显著减少，代码一次性正常通过的几率也会大幅度提升。
 
3. 继续学习如下的特性，并在工作中应用，以提高代码的可读性。
  * case class
  * pattern match
  * trait mixing
  
4. 不要轻易尝试如下特性。这些特性功能很强大，但也是导致scala背负着“宇宙最复杂的语言”的最重要的原因。我的建议是，不充许在应用层中使用，只充许在基础库中出现，并且编程者必需有一定的Scala水平级别。
  * implicits
  * type parameter(包括 协变、逆变、上届、下届、上下文界定、视图界定）
  * 类型路径
  * macro & scala reflect
  事实上，只要不使用这些“复杂”的特性，scala会是一门非常简单的编程语言。
  
我们会在下一篇《怎么避免把Scala写成Java》中介绍你需要开始规避的一些Java惯性，只有这样，我们才能让我们的代码质量进一步的提升。