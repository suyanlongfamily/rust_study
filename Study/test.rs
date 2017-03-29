fn main() {
    println!("123");
    let (a, b) = (1, 3);
    println!("a={},b={}", a, b);
    let mut x = 5;
    x = 123;
    println!("{}", x);
    println!("-----{}", add(1, 22));

    // 函数指针 f 后面是跟的类型，即函数对象（原型）类型
    // let f :fn(i32)->i32;//定义一个指定类型但是未初始化的变量（函数指针或者叫函数对象）
    // let f :fn(i32)->i32 = puls_one ;//定义一个指定类型并且初始化的变量
    let f = puls_one; //定义一个未指定类型但是未初始化的变量（函数指针或者叫函数对象）
    println!("---{}", f(123)); //函数对象的高级用法。

    loop {
        println!("123123");
    }

    
    diverges();

}

// 加法函数
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn puls_one(i: i32) -> i32 {
    i + 1
}
// let f:fn(i32) ->i32 = puls_one;
// // let f = puls_one;不能在文件作用域内使用语句。只能是表达式。或者函数定义。
// println!("---");

// 发散函数
fn diverges() -> ! {
    panic!("This function never returns!");
}