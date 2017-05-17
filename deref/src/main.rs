
use std::rc::*;

fn main() {
    println!("Hello, world!");
    let varcon = 2;
    println!("----{}---", varcon);
    let ni = 2;
    // 诺斯费都杀掉

    let s = "hello";
    println!("length: {}", s.len());
    println!("length: {}", (&s).len());
    println!("length: {}", (&&&&&&&&&&&&&s).len());

    println!("----length: {}", str::len(&s)); //注意这种形式可以的。

    // 自动deref的规则是，如果类型T可以解引用为U，即T: Deref<U>，则&T可以转为&U。


    let s = Rc::new(String::from("hello"));

    println!("length: {}", s.len());
    // println!("length: {}", s.deref().len());
    // println!("length: {}", s.deref().deref().len());

    println!("length: {}", (*s).len());
    println!("length: {}", (&*s).len());
    println!("length: {}", (&**s).len());


}

