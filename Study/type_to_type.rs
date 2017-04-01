

//类型转换，rust,做什么都是显示，不会有隐式转换。
// 不过都是使用as，显示的转换。并且遵循C语言规则。
//这里顺便说一下，rust，其实是在C语言的基础上发展出来的，不是C++


fn main() {
    let foo = 100;
    let bar = foo; //基本类型copy，可以调试查看。
    println!("{},{}", foo, bar);

    let val: Vec<i32> = vec![123123, 312, 123, 123];

}
