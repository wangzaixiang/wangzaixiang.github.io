+++
title = "the 1brc program"
description = "本文通过 1brc 这个编程挑战，来展示如何一步一步的进行代码优化，实现极具挑战的性能之旅"
date = 2025-02-06
draft = false
template = "blog/page.html"

[extra]
toc = true
+++

# About 1brc
[1brc](https://github.com/gunnarmorling/1brc) 编程挑战原本是一个使用 Java 处理 10 亿行文本数据，并进行汇总统计的挑战，这个程序的逻辑并不复杂：
1. 读取并分析文本数据，其格式如下：
   ```
    Hamburg;12.0
    Bulawayo;8.9
    Palembang;38.8
    St. John's;15.2
    Cracow;12.6
    Bridgetown;26.9
    Istanbul;6.2
    Roseau;34.4
    Conakry;31.2
    Istanbul;23.0
   ```
   每一行包括一个城市名和一个气温数值，以分号分隔。
2. 统计每一个城市的最高气温、最低气温、平均气温，并输出到标准输出。

程度的挑战在于，需要处理 10亿 行的文本数据（约13.7GB），到底可以在多快的时间内完成这个任务呢？不同于其他的编程挑战，会限定有限的内存、CPU 等资源，
1brc 官方给定的运行环境是 32 core AMD EPYC™ 7502P (Zen2), 128 GB RAM, 还是一个非常强大的机器。本文所使用的执行环境是 M1 Max(Macbook Pro 2021):
8性能核心，2节能核心，64GB RAM，也是一款非常强大的机器。这个挑战是在现代CPU和充裕的内存资源下，到底可以在多快的时间内，完成10亿行文本数据的处理。

原项目是使用Java进行挑战，不过在本文中，我会尝试基于 Rust 来完成这个挑战，毕竟，Rust 距离 CPU 更近，有更多的空间可供我们腾挪。当然，对我来说，也是一次
学习 Rust 的机会。

# ver1: 原始版本 108s

```rust
use crate::MEASUREMENT_FILE;
use std::collections::HashMap;
use std::io::BufRead;

#[allow(dead_code)]
#[inline(never)]
pub fn ver1() -> Result<HashMap<String,(f32,f32,f32)>, Box<dyn std::error::Error>> {

    let file = std::fs::File::open(MEASUREMENT_FILE)?;
    let reader = std::io::BufReader::new(file);

    struct Item {
        min: f32,
        max: f32,
        count: i32,
        sum: f32,
    }
    let mut hash: HashMap<String, Item> = std::collections::HashMap::new();
    for line in reader.lines() {
        // name;value
        let line = line?;
        let parts = line.split(';').collect::<Vec<&str>>();
        let name = parts.get(0).unwrap();
        let value = parts.get(1).unwrap().parse::<f32>()?;

        match hash.get_mut(*name) {
            Some(item) => {
                item.count += 1;
                item.sum += value;
                item.min = item.min.min(value);
                item.max = item.max.max(value);
            }
            None => {
                let item = Item {
                    min: value,
                    max: value,
                    count: 1,
                    sum: value
                };
                hash.insert(name.to_string(), item);
            }
        }
    }


    let result = hash.iter().map(|(name, item)| {
        (name.clone(), (item.min, item.max, item.sum / item.count as f32))
    }).collect();

    Ok( result )
}

```

作为一个基础版本，这段代码可谓是中规中矩：
1. 使用 `BufReader` 逐行读取文件，然后使用 `split` 分割字符串。（如果不使用 BufReader，估计速度还会慢1个数量级）
2. 使用 HashMap 来处理聚合数据。
3. 单线程。（本案例全部基于单线程来进行分析）

性能数据：
1. 耗时：107.9s
2. IPC: 4.91 (看起来还很不错)
3. Branch misses: 0.64%
4. [samply](https://github.com/mstange/samply) profile [link](https://share.firefox.dev/3WMkb80):
   ![img.png](img.png)
   ```bash
   samply load profiles/profile-v1.json  
   # 然后使用 firefox 打开 http://localhost:3000 端口即可查看 samply 的 profile 分析图表
   ```
   - read_line: ~46s
     - fill_buf: ~2.76s 这里是真实与OS打交道，读取文件内容的开销（真实的读取文件的开销并不高，远低于其他部分）。 
     - memchr: ~20.5s 是查找换行符的开销。
     - Vec::extend_from_slice: ~12.75s 是复制数据到 Vec 的开销。
     - from_utf8: ~7s, 是处理 UTF-8 的开销。
   - `line.split(';').collect::<Vec<&str>>()` ~22s
     - Vec 分配内存的时间： ~6.5s
   - `hash.get_mut(*name)` ~12s，Rust 的 HashMap 采用了 SIMD 加速，比常规的 HashMap 快了很多。
   - Drop Vec<&str> ~11s: 释放内存的开销居然这么高，如果再加上 Vec/String 的分配开销（~13s），花在内存分配释放上的时间(~16s)，不容小嘘。
   - Drop String ~7.7s
   - `parts.get(1).unwrap().parse::<f32>()?` ~ 6.6s，目前还不是我们的瓶颈点。

第一个基线版本出来了，看起来，read_line 是目前最大的瓶颈，其中真实的 IO 开销并不高（本次测试的电脑有64G内存，因此，文件的内容大概率是全部在 Page Cache中的，
并不会涉及到真实的IO，目前来看，2.76s 读取完全部文件内容，看起来还是比较快的），而花在 查找换行符，复制数据（String）、解析UTF-8、分配和释放内存的耗时占比很比。

# ver2, 89s
在 ver1 中，我们发现 read_line 中 `Vec::extend_from_slice` 上花费了12.75s 的时间，原因是我们每次读取并返回一个新行（String），处理完成后
就释放掉这个该字符串（～7.7s），如果我们不重复分配、释放内存，而是复用一个 String 呢？简单的做如下修改：[详细代码](https://github.com/wangzaixiang/onebrc_rust/blob/master/src/ver2.rs)：
![diff_ver2_ver1.png](diff_ver2_ver1.png)

性能数据：
1. 耗时：89.3s (-17.6s, +16.3%)
2. IPC: 4.57
3. Branch misses: 8.4%
4. [samply profile](https://share.firefox.dev/4htrWrC)

看起来不错，几行代码的修改，就提升了16.3%的性能，查看 samply profile, 计划要消除的耗时都如预期的减少了。
1. 内存分配和释放的成本，并没有那么的廉价（当然，这个案例可是10亿次循环，把这个成本给放大了）。对于parser 处理 AST 这样的场景，多遍遍历，
   大量的小对象的分配和释放，这个成本是非常高的。
2. 在 JVM 中，对象的分配和释放更是无处不在，由于没有 value object，所有的对象都在 heap 中分配，而更高效的语言，一定需要 value object,并
   优先在 stack（or in-place）中分配。相比 heap， stack allocate 可以认为是零成本的。当然，JVM的GC机制，可能会比C/Rust等语言在分配/释放内存
   上更加高效：一是由于内存整理，避免了碎片的存在，分配内存成本更低。二是几种的释放，尤其是分代GC下年轻代的复制回收机制，成本相比逐一的回收会有
   显著的提升。但无论如何，如果能避免分配和释放，才是最佳的选择。

# ver3, 65s

消除了 read_line 的内存分配和释放的成本，我们的瓶颈点又转移到了 `line.split(';').collect::<Vec<&str>>()` 上，这个操作的成本是22s，其主要成本也是
花费在 Iterator 的遍历和 Vec 的分配和释放上。由于每一行文本格式比较简单，因此我们重新优化一下我们的代码 [ver3完整连接](https://github.com/wangzaixiang/onebrc_rust/blob/master/src/ver3.rs)：
![img_1.png](ver2_collect.png)

![img_1.png](ver3_code1.png)
![img_1.png](ver3_code2.png)

性能数据：
1. 耗时：65.5s (vs ver2: -23.8s, +26.6%) (vs ver1: -42.4s, +39.4%)
2. IPC: 4.12
3. Branch misses: 0.94%
4. [samply profile](https://share.firefox.dev/4hqukPN)

ver2/ver3 两轮优化后，在 samply profile 中，已经看到没有明显的内存分配、释放的开销了，性能的整体提升达到了39.4%。为我们的优化成就鼓掌。

收获：
1. 在这两轮的优化中，samply 的贡献是最大的，他为我们提供了直观的性能数据。
2. 大部份应用在早期都存在“低垂”的果实，往往只需要较少的代价，就可以取得巨大的性能提升。在实际应用中，这些低垂的果实，可能会获得成倍、
   数十倍的性能提升，而代价则非常微小。

# ver4, 51s

对 ver3 的 profile 数据进行分析发现，&str::from_utf8 花费了 ~9s 的时间，由于我们的输入文件是一个格式良好的文件，在本次性能挑战中，可以忽略
UTF8 检查的成本（当然，在实际应用中，需要权衡输入检查的必要性，不一定能够妥协，不过在一个复杂系统中，应该尽可能的在最外层进行足够的输入校验，避免在
后续内部的处理中重复的进行正确性的检查：即浪费了计算资源，也增加了代码复杂性，参考我在这篇 [blog 中所说的 DBC](@/blog/2024-10-07-complexity/index.md)）

调整后的代码：[完整链接](https://github.com/wangzaixiang/onebrc_rust/blob/master/src/ver4.rs)
```rust
fn read_line<'a>(reader: &mut std::io::BufReader<std::fs::File>, line: &'a mut Vec<u8>) -> Result<Option<(&'a [u8], i64)>, Box<dyn std::error::Error>> {
    let pos = line.len();
    let n1 = reader.read_until(b';', line)?;
    if n1 > 0 {
        // let part1 = &line[pos..pos+n1-1];
        let n2 = reader.read_until(b'\n', line)?;
        if n2 > 0 {
            let part1 = &line[pos..pos + n1 - 1];
            let part2 = &line[pos + n1..pos + n1 + n2 - 1];
            let mut value = 0i64;
            let mut sign = 1;
            for i in part2.iter() {
                if *i == b'.' {
                    continue
                } else if *i == b'-' {
                    sign = -1;
                } else {
                    value = value * 10 + (i - b'0') as i64;
                }
            }
            Ok(Some((part1, value * sign)))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
```
![img_1.png](ver4_code1.png)

性能数据：
1. 耗时：51.2s (vs ver3: -14.3s, +21.8%) (vs ver2: -38.7s, +43%) (vs ver1: -56.7s, +52.6%)
2. [samply profile](https://share.firefox.dev/40ObCun)

经过 ver4 的优化后，samply profile 中的火焰图变得更加的简单，目前主要的开销包括：
- `std::io::BufRead::read_until` ~ 32s 以 `, \n` 作为分隔符逐一读取 &str
- `std::collections::hash::map::HashMap::get_mut` ~ 15s

低垂的果实似乎已经采摘完毕，这两个感觉都不好进行进一步的优化了。那么我们还能做些什么呢？

未完待续...