// 所有权系统
// 所有权是针对引用类型的，
//函数调用时传递引用类型会发生所有权的转移
// 所有基本类型都实现了Copy trait，因此他们的所有权并不像你想象的那样遵循“所有权规则”被移动。
//Copy类型,我们会在trait部分讨论如何编写你自己类型的Copy。
// 我们已经知道了当所有权被转移给另一个绑定以后，你不能再使用原始绑定。然而，有一个trait会改变这个行为，它叫做Copy。
// 我们还没有讨论到 trait，不过目前，你可以理解为一个为特定类型增加额外行为的标记。例如：
fn main() {
    foo();
    let mut x = 45;
    {
        let y = &mut x;
        *y += 2;
    }
    println!("---{}", x); //在println! 里面会生成一个不可变的引用。否则会导致与y所处的作用域冲突。
    // -----------------------------------
    let mut mut_x = String::from("hello");
    let mut_x1 = &mut_x;
    let mut_x2 = &mut_x;
    // let mut_x3 = &mut mut_x;可变引用，
    // 关于引用的定义一定要理解。
    // rust中，不能同时拥有可变与不可变引用。
}

fn foo() {
    let v = vec![1, 2, 3];
    for var in v {
        println!("{}", var);
    }

}
