struct NULL;

trait DoubleNull<T> {
    fn drop(self, a: T);
}

impl<T> DoubleNull<T> for NULL
    where T: std::fmt::Display//这种也可以的，可以在实现的时候添加约束。
{
    fn drop(self, a: T) {
        println!("---{}", a); //类是这种泛型，突然是先类型检测，然后才是编译期间对模版进行展开。
        //关于println!宏需要实现Debug与Display 这两个接口，也是醉了。不像C++一样。对模版展开以后才进行检查。  
    }
}

fn main() {
    foo(12);
    let val = 123;
    let Null_val = NULL {};
    Null_val.drop(val);
    // Null_val.DoubleNull(val);
}

use std::fmt::Debug;
//前面表示对泛型参数的约束。
fn foo<T: Debug>(values: T) {
    println!("{:?}", values);
}