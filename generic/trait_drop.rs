
// struct Empty;
// struct NUll;
// trait DoubleDrop<T> {
//     fn double_drop(self, _: T);
// }

// impl<T, U> DoubleDrop<T> for U {
//     fn double_drop(self, _: T) {}
// }

// fn main() {
//     let empty = Empty;
//     let null = NUll;

//     empty.double_drop(null);
//     // empty;
//     // null;
// }

//1、impl YYY :一般就是为YYY结构体，添加成员函数。
//2、impl XXX for YYY 在YYY的基础上实现XXX，即为YYY实现XXX。
trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}
impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

//模版编译，编译期间进行展开，代码膨胀。
// fn do_something<T: Foo>(x: T) {
//     x.method();
// }

// fn main() {
//     let x = 5u8;
//     let y = "Hello".to_string();
//     println!("xxx {:?}", do_something(x));
//     println!("yyy {:?}", do_something(y));
// }

//动态分发，虚函数表的移动与绑定与C++一样。
fn do_something(x: &Foo) {
    x.method();
}

fn main() {
    let x = 5u8;
    // x.method();//也可以对先有的类型进行扩展。
    // do_something(&x as &Foo);
    do_something(&x);
}
