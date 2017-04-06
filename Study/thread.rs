use std::thread;
static NTHREANDS: i32 = 10; //静态变量

fn main() {
    println!("----thread begin....");
    let mut children = vec![];
    for i in 0..NTHREANDS {
        children.push(thread::spawn(move || {
                                        println!("this is thread number {} ", i);
                                    }));
    }
    for child in children {
        // 等待线程到结束。返回一个结果。
        let _ = child.join();//调用子线程等待。 
    }
    println!("----thread end....");
}