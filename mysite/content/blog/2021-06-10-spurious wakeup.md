+++
title = "无端唤醒"
description = "Object.wait 等方法有一个很特别的场景：spurious wakeup, 会导致一些非常低频的Bug."
date = 2021-06-10
draft = false
template = "blog/page.html"
+++

# 无端唤醒（spurious wakeup）

前些时间，我们自研的一款 Redis 驱动，有反应存在如下的现象：偶尔会出现 redis 发送命令，在没有超时的情况下，却返回了“没有收到返回结果”的情况，虽然几率很低，但也是断断续续出现了几次，应该是一个非常隐藏的 BUG。

为了定位这个问题，我们将相关的逻辑进行了简化，编写了如下等效的代码，检测是否会重现问题。

```java
package demo;

import java.util.concurrent.BlockingQueue;
import java.util.concurrent.LinkedBlockingQueue;
import java.util.concurrent.atomic.AtomicInteger;

public class Test1 {

    BlockingQueue<Task> queue = new LinkedBlockingQueue<>(100);

    class Task {
        String request;
        String response;
    }

    class Consumer extends Thread {

        public Consumer(){
            setDaemon(true);
        }

        @Override
        public void run() {
            while(true){
                try {
                    Task task = queue.take();

                    synchronized (task) {
                        task.response = task.request.toUpperCase();  
                        task.notify();
                    }
                }
                catch(InterruptedException ex){
                    ex.printStackTrace();
                }
            }
        }
    }

    class Producer extends Thread {

        @Override
        public void run() {
            int count = 0;
            while(true){
                final Task task = new Task();
                task.request = "Hello";  // visible to others?
                boolean offOK = queue.offer(task);  // check offer
                if(offOK == false) {
                    System.out.println("offer failed, retry");
                    continue;
                }

                boolean waitOk = false;
                long start, end;

                synchronized (task){
                    if(task.response == null) {
                        try {
                            task.wait(60_000);
                            waitOk = true;

                            if(task.response == null){ // do volatile required?
                                System.out.println("reponse is null waitok = " + waitOk + " waitTime:" + count);
                                System.exit(1);
                                break;
                            }
                        }
                        catch(InterruptedException ex){
                            ex.printStackTrace();
                        }
                    }
                }

                count += 1;
                if(count % 1000_000 == 0) {
                    System.out.println("runs " + count/1000_000 + "M turns");
                }
            }
        }
    }

    void runTest(){
        new Producer().start();
        new Consumer().start();
    }

    public static void main(String[] args) {
        new Test1().runTest();
    }

}

```

这个代码是一个简单的生产者、消费者模型：
1. 生产者：往队列中提交一个消息（Task），等待消费者处理完后唤醒生产者。期望 Task 中的response已经有正确的值。
2. 消费者：等待队列的消息，处理完一条消息后，唤醒生产者。

运行如上的测试代码，很快就可以重现问题，可能在5百万次至1-2亿次循环后，程序打印 `reponse is null `后退出。

最开始的时候，我们的怀疑点是：Consumer 线程中执行的 `task.response = task.request.toUppercase()` 结果对 Producer 线程不可见。（但按照 synchronize 的语义，在 synchronize 块monitor exit离开时，数据修改对其他线程是可见的），我们尝试将 response 字段修饰为 volatile， 但结果仍然会出现同样的问题，因此，问题，并不处在 synchronize/volatile 引起的数据可见性方面。

> 关于JMM的学习，强烈推荐：[深入理解Java内存模型](https://www.infoq.cn/minibook/java_memory_model?utm_source=related_read&utm_medium=article)，以及对照查看 [JMM官方文档](https://docs.oracle.com/javase/specs/jls/se15/html/jls-17.html#jls-17.4)
>

因为这段代码能够更好的进行问题的重现（而原 Redis 驱动要重现这个问题，则需要数天的时间），因此，我们对这段代码略作改造，在出现异常的代码点上，添加了一些调试代码，结果发现了一个有意思的现象：
- 当生产者进入到 `System.exit`时， 相应的消费者还没有进入到 `synchronzie` 块中。

也就是说，当前的Consumer并没有开始消费 Task，自然也没有进行 notify 调用，但Producer的 wait 却在没有任何异常的情况下，已经返回了。

这个时候，再仔细阅读一下 Object.wait 的文档：
>
> A thread can also wake up without being notified, interrupted, or
> timing out, a so-called <i>spurious wakeup</i>.  While this will rarely
> occur in practice, applications must guard against it by testing for
> the condition that should have caused the thread to be awakened, and
> continuing to wait if the condition is not satisfied.  In other words,
> waits should always occur in loops, like this one:
> <pre>
>   synchronized (obj) {
>       while (&lt;condition does not hold&gt;)
>           obj.wait(timeout);
>       ... // Perform action appropriate to condition
>   }
> </pre>
> (For more information on this topic, see Section 3.2.3 in Doug Lea's
> "Concurrent Programming in Java (Second Edition)" (Addison-Wesley,
> 2000), or Item 50 in Joshua Bloch's "Effective Java Programming
> Language Guide" (Addison-Wesley, 2001).
>

看来之前都忽略了这个 spurious wakeup (无端唤醒)。按照这个文档的指导，上述的代码应该重写为：

```java

   synchronized(task) {
        long begin = System.currentTimeMillis();
        while(task.response == null) {
            try {
                long now = System.currentTimeMillis();
                long waitTime = begin + 60_000L - now;
                if(waitTime > 0) 
                    task.wait(waitTime);
            }
            catch(InterruptedException ex){
                ex.printStackTrace();
            }
        }
    }

    if(task.response == null) { // do volatile required?
        System.out.println("reponse is null waitok = " + count);
        System.exit(1);
        break;
    }
```

## 小结
1. Object.wait 会被无端唤醒，这个知识点较为冷门。如果没有关注到这个点的话，wait/notify 很可能会出现小几率的是失败（在上亿级调用的场景下，则近乎是大几率事件）。
2. 对小几率出现的并发问题，需要严重以待，这些事件几乎肯定是程序的Bug，出现一次，就意味着一定会有第二次和更多次。
3. 伴随这个问题的解决，我几乎重温了一遍 JMM，在逐一排查的过程中，似乎又深入理解了一些。这些案例带给我们的帮助，相比单纯的学习要效果好很多。