// 默认 Rust 中，对一个资源，同一时刻，有且只有一个所有权拥有者。
// Rc 和 Arc 使用引用计数的方法，让程序在同一时刻，实现同一资源的多个所有权拥有者，多个拥有者共享资源。

// Arc 是原子引用计数，是 Rc 的多线程版本。Arc 通过 std::sync::Arc 引入。
// 它的特点：
// Arc 可跨线程传递，用于跨线程共享一个对象；
// 用 Arc 包裹起来的类型对象，对可变性没有要求；
// 一旦最后一个拥有者消失，则资源会被自动回收，这个生命周期是在编译期就确定下来的；
// Arc 实际上是一个指针，它不影响包裹对象的方法调用形式（即不存在先解开包裹再调用值这一说）；
// Arc 对于多线程的共享状态几乎是必须的（减少复制，提高性能）。

use std::rc::Rc;
use std::rc::Weak;
use std::sync::{Arc, Mutex};
use std::thread;
use std::mem;

fn main() {
    println!("------------begin-------------------");

    let five = Rc::new(5);
    let five2 = five.clone();
    println!("five values = {}", five.as_ref());

    let mut share_value = Arc::new(20);
    let s_v = share_value.clone();
    let joinhandl = thread::spawn(move || {
        //第一种修改形式。
        {
            let mut k = *share_value;
            k = 20;
            println!("---------kkk-{:?}---------", k);

        }
        // 第二种修改形式。
        // share_value.
        // let data = Arc::get_mut(&mut share_value);//没有其他的指针时，会返回。
        // *data = 2;
        // println!("----------{:?}---------", data);
        //   println!("----------{}---------", s_v.as_ref());
    });
    // share_value.clone();这个不能使用了，因为所有权转移了，这个也就是为什么要 let share_value_oth = share_value.clone();这样做
    // 时clone一份Arc的对象，不是包裹指向的值。其实，这个智能指针都是指向的同一份值，仅仅时智能指针对象clone了一份。
    // s_v.clone();

    let mut kk = *s_v;
    kk = 100;
    println!("---------kk-{:?}---------", kk);
    let mut x = Arc::new(3);
    *Arc::get_mut(&mut x).unwrap() = 4;
    assert_eq!(*x, 4);
    mem::drop(x);
    joinhandl.join().unwrap();
    println!("------------end-------------------");
}
