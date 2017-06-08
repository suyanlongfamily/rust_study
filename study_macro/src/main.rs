
extern crate time;

macro_rules! say_hello {
    () => (
        println!("hell world");
    );
}

// #[macro_use];

fn main() {
    println!("----{:?}-----", time::get_time());
    println!("----{:?}-----", time::now());
    println!("----{:?}-----",  time::now_utc());
    let str = "%h:%m:%s:%n:%u";
    println!("----{:?}-----", time::strftime(str,&time::now()).unwrap());
    println!("----{:?}-----", time::empty_tm());


    say_hello!();
}



// 使用大括号的时候，大括号后面没有分号；
// 而使用（）和［］的时候，后面有一个分号。
// ❌如果大括号｛｝有分号；或者（）和［］没有分号都会编译错误。

// ⚠ 注意：

// 另一个需要注意的是，宏的输入参数是包含在宏内容体内的！！！