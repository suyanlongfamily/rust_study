// use std::marker::Send
use std::thread;
#[derive(Debug,Copy,Clone)]
struct Name {
    old: u32,
    year: u32,
}

fn main() {
    println!("Hello, world!");
    Send_test();


}


fn Sync_test(){

}

fn Send_test() {
    let mut count = 10;
    //由于是copy类型，所以move到另一个执行体中，是新的值，与原来的是不是同一个，同时也是Send约定。
    //而没有实现copy类型的，就是转移所有权了。原来的变量就不能再使用了。
    //move 后的变量，要么时转移了所有权，要么copy了值（需要实现copy\clone\等特性）。
    let mut stu = Name { old: 12, year: 32 };
    let handle = thread::spawn(move || loop {
                                   println!("-----childen thread--");
                                   if let Some(0) = Some(count) {
                                       return;
                                   }
                                   stu.old = 1233434;
                                   count = count - 1;
                               });

    handle.join();
    println!("---main ---{:?}----", count); //不变，
    println!("---main ---{:?}----", stu); //不变，

}



// 背景： Send 和 Sync
// 并发很难解释清楚。在Rust中，我们由一个强大的静态的类型系统来帮助我们理解我们的代码。
// Rust本身给我们两个特性，帮助我们实现并发编程。

// Send
// 第一个类是 Send. 当类型 T 实现了 Send, 它告诉编译器这个类型的实例的所有权可以在多个线程之间安全传递。
// 实施强制的限制条件是很重要的。例如，如果我们由一个通道在两个线程之间，我们可能会想在两个线程之间传递数据。
// 因此，我们要保证传递的数据类型要实现 Send 特性。
// 相反，如果我我们用FFI包裹了一个线程不安全的类库，我们不会去实现 Send, 编译器会帮助我们确保它不会离开当前线程。

// Sync
// 第二个特性是 Sync. 当一个类型实现了 Sync, 它向编译器表明这个类型的数据在多线程并发是不可能导致内存的不安全。
// 例如，由原子引用计数的不可变数据的共享是线程安全的。Rust提供了一个类型 Arc<T>, 它实现了 Sync, 因此它可以在线程之间共享。
// 这两个特性运行你使用类型系统来保证你代码在并发情况下的所有权。在解释为什么之前，让我们先创建一段并行的Rust代码。

// 一个（Send标记）可以安全转移多有权，
// 一个（Snyc标记）同一份数据可以在多一个执行体中，可以安全的共享内存数据，
// 注意，实现这种约定的类型，才有资格。
// 推论一下：
// 1、Sync 标记的类型，可以安全共享，所以它的&T 是Send约定，不然怎么发送到另一个线程体中？即：T：Sync => &T:Send
// 2、Copy + Sync = Send; 看上面的解释
// 3、当 T: Send 时，可推导出 &mut T: Send；？？？//
// 4、当 T: Sync 时，可推导出 &mut T: Sync；//这个思考Arc智能指针，就明白了。
// 5、当 &mut T: Send 时，不能推导出 T: Send；这个确实是的。所有权不一定能转移。


// （注：T, &T, &mut T，Box<T> 等都是不同的类型）
// 具体的类型：
// 原始类型（比如： u8, f64），都是 Sync，都是 Copy，因此都是 Send；
// 只包含原始类型的复合类型，都是 Sync，都是 Copy，因此都是 Send；
// 当 T: Sync，Box<T>, Vec<T> 等集合类型是 Sync；
// 具有内部可变性的的指针，不是 Sync 的，比如 Cell, RefCell, UnsafeCell；
// Rc 不是 Sync。因为只要一做 &Rc<T> 操作，就会克隆一个新引用，它会以非原子性的方式修改引用计数，所以是不安全的；
// 被 Mutex 和 RWLock 锁住的类型 T: Send，是 Sync 的；
// 原始指针（*mut, *const）既不是 Send 也不是 Sync；
// Rust 正是通过这两大武器：所有权和生命周期 + Send 和 Sync（本质上为类型系统）来为并发编程提供了安全可靠的基础设施。使得程序员可以放心在其上构建稳健的并发模型。
// 这也正是 Rust 的核心设计观的体现：内核只提供最基础的原语，真正的实现能分离出去就分离出去。并发也是如此。

// 理论上并行和语言并没有什么关系，所以在理论上的并行方式，都可以尝试用Rust来实现。
// Rust的一大特点是，可以保证“线程安全”。
// 而且，没有性能损失。更有意思的是，Rust编译器实际上只有Send Sync等基本抽象。
// 而对“线程” “锁” “同步” 等基本的并行相关的概念一无所知，这些概念都是由库实现的。
// 这意味着Rust实现并行编程可以有比较好的扩展性，可以很轻松地用库来支持那些常见的并行编程模式。 
