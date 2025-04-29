+++
title = "M1 CPU 分支预测失误对性能的影响测试"
description = "在 M1 CPU 上进行的分支预测失误性能测试"
date = 2025-04-29
draft = false
template = "blog/page.html"

[extra]
toc = true
+++

# M1 CPU 分支预测失误对性能的影响测试


| test case | IPC   | branch-misses | ns/iter | description |
|-----------|-------|---------------|---------|-------------|
| test1     | 3.996 | 4.88e-3       | 0.775   |             |
| test2     | 4.596 | 6e-7          | 0.678   |             |
| test3     | 1.908 | 2.9e-2        | 1.555   |             |

假设分支预测成功时 ns/iter 为 x，失败时为 y，则：

x = 0.678
(0.971 * x + 0.029 * y) = 1.555
y = (1.555 - 0.971 * x) / 0.029 = (1.555 - 0.971 * 0.678) / 0.029 = 30.91

y/x = 30.91 / 0.678 = 45.6
即：1次分支预测失败的成本约等于 45.6 次分支预测成功的成本。

1. 测算


```
	rbit   x11, x10                 
	clz    x11, x11                 
	lsl    x12, x9, x11             
	add    x25, x11, x25            
	add    x24, x24, #0x1           
	mov    x1, x24                  
	mov    x0, x25                  
	bics   x10, x10, x12            
	b.ne   0x1000028a0                ; <+3112> [inlined] core::num::<impl u64>::trailing_zeros at uint_macros.rs:162:20
```

y = (0.775 - 0.99512 * 0.678) / 0.00488 = 20.55


