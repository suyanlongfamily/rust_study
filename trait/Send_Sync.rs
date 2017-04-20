// 具体的类型：
// 原始类型（比如： u8, f64），都是 Sync，都是 Copy，因此都是 Send；
// 只包含原始类型的复合类型，都是 Sync，都是 Copy，因此都是 Send；
// 当 T: Sync，Box<T>, Vec<T> 等集合类型是 Sync；
// 具有内部可变性的的指针，不是 Sync 的，比如 Cell, RefCell, UnsafeCell；
// Rc 不是 Sync。因为只要一做 &Rc<T> 操作，就会克隆一个新引用，它会以非原子性的方式修改引用计数，所以是不安全的；
// 被 Mutex 和 RWLock 锁住的类型 T: Send，是 Sync 的；
// 原始指针（*mut, *const）既不是 Send 也不是 Sync；

// Rust 正是通过这两大武器：所有权和生命周期 + Send 和 Sync（本质上为类型系统）来为并发编程提供了安全可靠的基础设施。
// 使得程序员可以放心在其上构建稳健的并发模型。
// 这也正是 Rust 的核心设计观的体现：内核只提供最基础的原语，真正的实现能分离出去就分离出去。并发也是如此。