//参数中 是 “_” 符号时的情况意义：这里代表函数内部没有使用他，可以用一个匿名的名词代替它
//返回值里面的泛型可以省略是什么意思？？？？这个是在错误错里面的result里面的东西。如下：
//Result 枚举类型是和Option枚举类型有区别的。看如下：
// enum Result<T, E> { //‘Result定义’
//     Ok(T),
//     Err(E),
// }
// Result是Option的更通用的版本，比起Option结果为None它解释了结果错误的原因，所以：
// type Option<T> = Result<T, ()>;
// 这样的别名是一样的（ )标示空元组，它既是()类型也可以是()值 ）。这一句话特别重要，相当于void类型。
// type Result<T> = result::Result<T, ParseIntError>;别名定义是什么意思？？？
// 其实想想，如果我们换做type Result = result::Result<T, ParseIntError>; 这样更容易理解了。


trait Drop<T> {
    fn drop(self,_:T); 
}

#[derive(Debug)]
struct Student {
    name:&'static str,
    old:i32,
    address:&'static str,
}

impl<T> Drop<T> for Student {
    fn drop(self,_:T){//这里面的"_"表示内部没有使用到，所以可以用这种特殊符号使用。
        println!("---------------释放资源，通过所有权的特性");        
    }
}


fn main() {
    println!("--------------begin---------");
    let stu = Student{name:"suyanlong",old:28,address:"杭州"};
    println!("{:?}",stu);
    stu.drop(12);
    println!("--------------end---------");
}