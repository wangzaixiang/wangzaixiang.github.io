+++
title = "Understand Svelte Rune"
description = "通过代码片段讲述 Svelte Runes 背后的原理"
date = 2025-03-26
draft = false
template = "blog/page.html"
+++

# What is Rune?
Svelte 5 是 Svelte 的一次大版本更新，其核心特性是引入了 runes 这个概念，在[这篇官方 blog](https://svelte.dev/blog/runes) 中，作者对 runes 进行了解释：

> rune:
> A letter or mark used as a mystical or magic symbol. (一种通过字符或者标记来表达某种特殊含义的符号)

在某些编程语言中，也有相似的概念：
- vimscript 中的变量前缀 

  ```text
  let g:ack_options = '-s -H'    " g: global
  let s:ack_program = 'ack'      " s: local (to script)
  let l:foo = 'bar'              " l: local (to function)
  let w:foo = 'bar'    " w: window
  let b:state = 'on'   " b: buffer
  let t:state = 'off'  " t: tab
  echo v:var           " v: vim special
  ```
  
- Ruby 中的变量前缀

  ```ruby
  $global_variable = 0
  @instance_variable = 0
  @@class_variable = 0
  CONSTANT = 0
  local_variable = 0
  ```

- go 语言中通过大小写来区分变量的可见性

  ```go
  package main

  import "fmt"

  var GlobalVariable = 0
  var globalVariable = 0

  func main() {
    fmt.Println(GlobalVariable) // 0
    fmt.Println(globalVariable) // 0
  }
  ```
  
通过使用某些前缀性标识字符，来表达某种特殊的含义，和 ROR(Rails on Ruby) 中的约定优于配置(Convention Over Configuration)的思想类似，
这种方式可以让代码更加易读，更加易于维护。但一般较少应用与底层编程语言中，因为一般来说，约定是上层的，在不同的环境中，可能会有不同的约定。而底层
语言一般不与特定的环境绑定，所以一般不会有这种约定。

# Svelte Rune

通过一个简单的代码示例，对比一下 Svelte 4 与 Svelte 5 的区别：

```html
// Svelte 4
    <script lang="ts">
        let count = 0;

        function increment() {
            count += 1;
        }
    </script>

    <button on:click={increment}>
        clicks: {count}
    </button>

// Svelte 5
    <script lang="ts">
        let count = $state(0);      // vs: let count = 0;

        function increment() {
            count += 1;
        }
    </script>

    <button on:click={increment}>
        clicks: {count}
    </button>
```

粗略的看，Svelte 5 的代码比 Svelte 4 几乎是一样的，只是在 count 的定义上，多了一个 `$state` 的调用。

1. Svelte 4 中，这段代码是一段完全标准的 JavaScript 代码，没有任何特殊的地方。但 Svelte 框架会这段代码进行 Reactive 变换处理。这个过程
   是完全隐式的。
   - 优点的是，对用户完全透明，现有的代码无需任何修改，就可以使用 Svelte 的 Reactive 特性。
   - 缺点也是：用户难以了解这段代码的实际运行语义，是 reactive 的还是普通的 JavaScript 代码。
   - 如果这段代码不是放在 .svelte 文件中，而是放在一个普通的 .js 文件中，这种不明确的语义就会更加明显。
2. Svelte 5 中，通过 `$state` 调用(下面会介绍到，这其实不是一个函数调用)，明确的表明了这个变量是一个 Reactive 变量。这样，用户就可以清楚的知道这个变量是一个 Reactive 变量。
   - 优点是：用户可以清楚的知道这个变量是一个 Reactive 变量。对这个变量的读写，会按照 reactive 机制来进行。（下面会介绍这个reactive 机制）
   - 缺点是：从非 reactive 的代码转换到 reactive 的代码，需要进行修改。

可以这么理解：Svelte 5 和 Svelte 4 具有几乎相同的运行时语义，而差异仅仅是在源代码中的表述方式不同：Svelte 4 通过完全隐式的方式（按照约定），
而 Svelte 5 通过显式的方式,使用 runes 来显示标记需要进行 reactive 处理的变量。

在支持 macro 的编程语言中，也会使用类似的方式来处理：
1. Rust 中的 `#[derive(Debug, Display)]` 注解，会自动为类型实现 Debug 和 Display trait。
2. Java 中的 Lombok 框架，对类型添加 `@Data` 注解，会自动为类型添加 getter/setter/hashCode/equals 等方法。
3. [Scala Binding](https://github.com/ThoughtWorksInc/Binding.scala) Scala 通过 macro 实现对代码的转换实现 reactive 机制。利用 macro 和 @html 注解。

参照上述的对比，我们可以这么理解，runes 是类似于 Macro 的一种机制，采用 现有语言的语法（在svelte中，使用了 `$state, $props, $derived, $effect, $bindable, $inspect, $host` 等标记函数）
来显示申明某种特定的语义（这个语义由 macro-expansion 机制进行实现，在这里，就是 svelte compiler, svelte compiler 相当于 Scala 中 的 Macro）

# How Svelte Rune works
Svelte 提供了 playground 可以方便我们进行实验，这个 playground 还提供了查看编译后生成的 JS 代码的功能，可以帮助我们理解 runes 背后的工作原理。

case: [Reactive assignments示例](https://svelte.dev/playground/reactive-assignments)

```html
<script>
	let count = $state(0);

	function handleClick() {
		count += 1;
	}
</script>

<button onclick={handleClick}>
	Clicked {count}
	{count === 1 ? 'time' : 'times'}
</button>
```

对应的编译后的 JS 代码(已手动添加上类型信息)：

```javascript
import 'svelte/internal/disclose-version';
import * as $ from 'svelte/internal/client';

// function handleClick() {
//		count += 1;
//	}
function handleClick(_, count: Source<number>) {
	$.set(count, $.get(count) + 1);
}

// template compiled
var root: () => Node = $.template(`<button> </button>`);

export default function App($$anchor: Node) {
	let count: Source<number> = $.state(0);  // let count = $state(0);
	var button = root(); 

	button.__click = [handleClick, count];   

	var text = $.child(button);

	$.reset(button);

    // Clicked {count}\n{count === 1 ? 'time' : 'times'}
	$.template_effect(() => $.set_text(text, `Clicked ${$.get(count) ?? ''}\n${($.get(count) === 1 ? 'time' : 'times') ?? ''}`));

	$.append($$anchor, button);
}

$.delegate(['click']);
```

1. let count = $state 声明了一个 reactive 变量（对应类型为 `Source<T>`）
2. 在源代码中，对变量 count 的访问 get/set 操作，被提升为 `$.get(count)` 和 `$.set(count, value)` 的调用。
3. 为什么要提升？
   - `$.get(count)` 提升后，在 effect 上下文中时，自动建立上下文对 count 的响应式监听。
   - `$.set(count, value)` 提升后，除了更新 count 的值外，还会触发对下游监听着的通知，从而实现响应式的传播。
4. 在这个示例中，只有 text node 建立了对 count 的监听，模版的其他部份（静态内容）并不关心 count 的变化，因此，当 count 发生变化时，
   并不是整个 template 都会更新，而只有 text node 会更新。（当然，目前来说，一个 text node 是作为整体的）
   - 从目前 svelte 生成的代码来看，在一个模版内的静态内容，即使只有部份发生了变化，仍然是所有静态内容都重新计算并刷新，这一块还有优化空间。
   - 如果一个 $state 变量在整个App中，没有修改点，其会退化为一个 普通变量。
   - 从这个示例来看，这里的 effect 依赖过程是完全编译期静态确定的，svelte 仍然是在编译期动态处理，如果调整到编译期静态建立依赖关系，是否
     会有更好的初始化性能？
5. 这个实现方式放在 scala.js 中似乎实现成本更低。这个只是一个猜测，还没有实际评估。

# 其他
对其他的 rune，后续进行分析。

总结： rune 可以理解为使用现有语法元素，来定义一种新的语言级特性（也可以理解为一种新的语法），是一个貌似函数，实际为语法的标记。