//  针对线程之间的通信提供了异步的通道（channel）。
//  通道允许两个端点之间信息的单向流动：Sender（发送端） 和 Receiver（接收端）。
//  看到这里move关键字，也是需要学习的。把不经常见到的关键字总结一下。收集起来。

use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
static NTHREADS: i32 = 3;

fn main() {
    // 通道有两个端点：`Sender<T>` 和 `Receiver<T>`，其中 `T` 是要发送消息的类型（类型标注是可有可无的）
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();
    for id in 0..NTHREADS {
        // sender 发送端可被复制
        let thread_tx = tx.clone();

        // 每个线程都将通过通道来发送它的 id
        thread::spawn(move || {
            // 此线程取得 `thread_tx` 所有权
            // 每个线程都在通道中排队,列出消息
            // （原文：The thread takes ownership over `thread_tx` Each thread queues a message in the channel）
            thread_tx.send(id).unwrap();//子线程发送消息到通道队列里面，仍是生产者-》消费者模型。

            // 发送是一个非阻塞操作，线程将在发送完消息后继续进行
            println!("thread {} finished", id);
        });
    }

    // 所有消息都在此处被收集
    let mut ids = Vec::with_capacity(NTHREADS as usize);//创建指定大小的容器。
    for _ in 0..NTHREADS {
        // `recv` 方法从通道中拿到一个消息
        // 若无可用消息的话，`recv` 将阻止当前线程
        ids.push(rx.recv());//rx，获取通道的消息，并压入容器内部。消费者
    }

    // 显示已发送消息的次序
    println!("{:?}", ids);
}