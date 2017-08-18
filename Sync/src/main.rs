
use std::thread;
use std::sync::{Arc, Mutex};

///	1、首选 & 这种借用肯定是不能夸线程被访问的。 borrowed value must be valid for the static lifetime... 这种借用必须要有生命周器
/// 2、所以就只能用Arc这种指针了。而Arc这种指针，其实到底层也是在转换成 *T 或者 *mut T类型，这样借用检查器就检查不到了，然后转变成堆里面的内存，或者说一开始就是在堆里面，
/// 3、其实一直都有一个误区，我们学习rust。rust里面的变量的空间应该都是在堆里面，包括我们let kk =100; 这种形式的定义， 也是在堆里面。只是关于释放是有借用检查器或者类型系统来维护的，
/// 所以，在学习的rust的时候一定不要有堆栈思想了。
#[derive(Default, Debug)]
struct Test {
    name: String,
    year: i32,
}

unsafe impl Sync for Test {}

fn main() {
    println!("Hello, world!");
    let kk = Test {
        name: "suyanlong".to_string(),
        year: 13,
    };

    let kk_ref = Arc::new(Mutex::new(&kk));
    let kkk = &kk;
    let thread_handler = thread::spawn(move || {
        println!("----{:?}", kk_ref.lock());

    });

    println!("----{:?}", kkk);
    thread_handler.join();

}
