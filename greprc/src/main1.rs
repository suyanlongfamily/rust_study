mod t_mod;
// use
use t_mod::*;

use std::collections::HashMap;


trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for i32 {
    fn area(&self) -> f64 {
        *self as f64
    }
}

use std::string::ToString;
use std::fmt;
#[derive(Debug)]
struct Name {
    values: String,
}

//这个又学习一招了，这个非常值得思考，思考rust的泛型、模板的真谛！！！
// 1、为什么可以编译过，但是运行时错误的！！！！，很值得思考。
// 2、这个表明了一个crate内，对一个泛型类型T进行trait实现，是非常值得学习的。
impl fmt::Display for Name {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //这种的调用方式也值得学习！！！！！！
        fmt::Display::fmt(self, f)
    }
}


fn main() {
    println!("Hello, world!");
    let val: Vec<i32> = vec![1, 2, 3, 4];
    let kk = val[0];
    kk.call_back();

    let string_values = Name { values: "suyanlong".to_string() };

    // let sss = string_values.to_string();

    //这个地方才是厉害了的！！！！注意。
    //这是，对当前crate的类型进行的trait实现.值得学习了！！！！
    // string_values.call_back();
    let kk: i32 = 23;

    let value = String::from("suyanlong");

    value.to_string();
    // kk.call_back();
    // 5.area();


    //--------------------------------------------------------------------------

    use std::env;
    println!("---{:?}", env::consts::ARCH);
    println!("---{:?}--", env::consts::FAMILY);
    //全局函数。
    println!("---{:?}---", env::args());
    println!("---{:?}---", env::args_os());

    for pat in env::args_os() {
        println!("---{:?}---", pat);
    }

    println!("---{:?}---", env::home_dir().unwrap());
    //所有的环境变量。
    for pat in env::vars() {
        println!("---{:?}---", pat);
    }

    println!("---{:?}---", env::current_exe().unwrap());
    println!("----{:?}-----", env::temp_dir());
    //这种引用场景可以用在程序初始化，或者建立应用程序环境变量，这样就不需要什么配置文件，配置文件弱爆了。
    println!("----{:?}-----", env::var("HOME").unwrap());
    //如下，这样我们也不需要什么配置文件了。
    env::set_var("suyanlong_test", "12");
    println!("----{:?}-----", env::var("suyanlong_test").unwrap());
    //好好看看标准文档，英语一定要努力学习！！！！！
    //区分两者的区别。

    for pat in env::vars_os() {
        println!("--{:?}--", pat);
    }

    println!("--{:?}--", env::args());
    println!("-------------------");
    println!("----{:?}---------", env::join_paths(env::current_dir()));

    use std::string::String;
    println!("-----------{:?}-------- {:?}-----------",
             env::args().count(),
             env::args().last()); //  env::args().()););



}
