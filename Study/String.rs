// 称作String的类型是由标准库提供的，而没有写进核心语言部分，它是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型。
// 当 Rustacean 们谈到 Rust 的“字符串”时，他们通常指的是String和字符串 slice &str类型，而不是其中一个。
// 这一部分大部分是关于String的，不过这些类型在 Rust 标准库中都被广泛使用。
// String和字符串 slice 都是 UTF-8 编码的。


fn main() {

    // 一、创建一个字符串，
    //默认导入String作用域，
    let str_val = String::new();

    //静态全局只转换。
    let str_init = "suyanlong";
    let str_String = str_init.to_string();

    //字面值转化。String::from和.to_string最终做了完全相同的工作
    let mut str_data = String::from("suyanlong");

    ///二、更新字符串 push_str()进行相加
    str_data.push_str("123");
    str_data.push_str("4456");
    println!("String values = {:?}", str_data);

    //运算符+ 进行操作。
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // /所有权进行转移了。Note that s1 has been moved here and can no longer be used
    println!("s3 ={:?}", s3);

    // 对于更为复杂的字符串链接，可以使用format!宏：
    let ss1 = String::from("tic");
    let ss2 = String::from("tac");
    let ss3 = String::from("toe");
    let ss = format!("{}-{}-{}", ss1, ss2, ss3);

    //字符串索引无法编译
    //字符串索slice
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{:?}", s);

    // 遍历字符串 chars()\bytes()
    // 总而言之，字符串还是很复杂的。


}
