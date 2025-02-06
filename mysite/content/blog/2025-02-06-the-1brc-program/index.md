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

# base0: 原始版本

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
4. [samply](https://github.com/mstange/samply) profile:
   ![img.png](img.png)
   - read_line: ~46s
     - read_line: ~37.7s
     - from_utf8: ~7s
   - `line.split(';').collect::<Vec<&str>>()` ~22s
   - `hash.get_mut(*name)` ~12s
   - Drop Vec<&str> ~11s
   - Drop String ~7.7s
   - `parts.get(1).unwrap().parse::<f32>()?` ~ 6.6s

第一个基线版本出来了，看起来，read_line 是一个很大的瓶颈，其开销包括了读取文件、处理分隔行，处理UTF8字符串等。