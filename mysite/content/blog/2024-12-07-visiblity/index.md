+++
title = "Visibility Makes Software Simplicity"
description = "visibility，是从另外一个角度来降低软件复杂性的方式。"
date = 2024-12-07
draft = true
template = "blog/page.html"

[extra]
toc = true
+++

在 [关于软件复杂性](@/blog/2024-10-07-complexity/index.md) 中，我们讨论了软件复杂性的来源，症状，分类，以及如何减低接口复杂性、降低实现复杂性的
一些方法。本文，从另外一个角度，即软件的可见性、透明性这个角度，来讨论如何降低软件的复杂性。

# 1. 可见性（visibility）、透明性(transparency)
在 Unix编程哲学 一书第六章中，专门讨论了软件的可见性、透明性。

如果用户能够预测到程序行为的全部或者大部份情况，并能建立简单的心理模型，那么这个程序就是透明的，因为可以看透程序究竟是如何工作的，即用户了解程序的工作原理，并建立对其的掌控度。

可见性则是对程序执行过程中的状态、数据和行为的可见性，这些可见性可以帮助用户：
- 学习并理解程序的工作原理。
- 为诊断问题提供帮助。
- 对程序的改进提供反馈。

# 案例

## 1. Chrome DevTools
在进行 WEB 开发时， DevTools 为开发者提供了丰富的信息，包括：
- DOM 树：对一个漂亮的页面，我们可以通过 DOM 树来学习其布局的方式、使用的CSS样式等。当我们自己的页面布局出现问题时，DOM Inspector 是解决这类问题的利器。
- Network: 我们可以看到每个请求的细节，包括请求头、响应头、请求体、响应体等。这对于调试网络请求、优化网络请求、诊断网络问题等都是非常有用的。
- Console: 日志信息为我们提供有用的帮助信息。
- Performance: 我们可以看到页面的性能指标，包括加载时间、渲染时间、资源加载时间等。这对于优化页面性能非常有用。
- 其他，包括 Storage, Cookies, WASM, Service Worker

## 2. Prometheus like System Monitor/Metric Monitor
对任何一个互联网运营系统，都需要建立一套完善的运维系统，而监控系统则是整个运维系统的核心。监控包括：
- 系统资源监控：CPU、内存、磁盘、网络等。
- 中间件监控：数据库、缓存、消息队列等。
- 服务监控：QPS、RTT、错误率等。
- 业务监控：订单量、用户量、交易量等。
- 其他监控：包括日志监控等。

一般的，可监控内容的完备性，及时性，准确性，以及配套的监控、告警、故障响应体系，是一个计算机系统监控运行的关键。想象一下，对某个具有成百上千个微服务
（上万个服务节点也并不夸张）的一个互联网系统， 在某个时间点，系统出现服务不可用故障时，你要快速的定位问题，找到故障原因，并快速采取措施，这是一件
多么困难的事情。

# 关键词

1. CLI vs GUI
2. Chrome DevTools
3. System Monitor/Metric Monitor
4. 文本可视化（Log）、图形可视化（Dashboard）
5. Text Protocol vs Binary Protocol
6. REPL, Notebook, IDE
7. Low Code, No Code.
8. UML Model
9. Playground, Code Snippet, Shoelace's Component Gallery.
10. 为软件添加可视化，看上去会增加软件开发的成本，但最终会有效的降低软件的复杂性。
    1. 可视化为用户（end user/developer/operator）提供高维的视角。
    2. 复杂的软件，更难以可视化，所以，面向可视化变成，会促使软件朝着更为简单化的方向发展，并暴露不合理的复杂性。
    3. 可视化的工具，把软件中的框架、主干简单化，把复杂的细节隐藏起来。
11. Simple or Easy
    - battery included
    - cheat sheet
    - AI natural language.
12. 透明度
    - 对维护者：提供更好的理解性
    - 故障定义：提供更好的诊断和定位
    - 对改进优化：提供度量和反馈，给出更有针对性的改进建议。（刻意练习）
    - 性能优化： Profile
    - 运营优化：监控、数据可视化、KPI改进。
13. 神经科学：人的大脑擅长理解图像，胜于理解文字等符号。

