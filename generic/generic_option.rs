// 标准库（std）中有个叫做 Option<T> （option 中文意思是“选项”）的枚举类型，
// 用于不存在是可能发生的情景
// 原文：An enum called Option<T> in the std library is used when absence is a possibility. ）。
// 它表现为以下两个 “options”（选项）中的其中一个：
// Some(T)：找到一个属于 T 类型的元素
// None：找不到相应元素
// 主要还是枚举的定义，不够熟练。

enum Name<T> {
    None, //这里的值代表什么呢？？？是一个数字吗？
    Vsome(T), //在rust里面是一个元组。元组的定义。
}

use Name::Vsome; //引入当前空间。枚举的引入，厉害了。
fn main() {
    let a = Vsome(100.22f32); //这个Some是关键字吗？还是全局函数？
    let a1 = Vsome(100.22f32);//Vsome 这一个元组对象，新类型，这样都可以的。
    let a2 = Vsome(100.22f32); //这样都可以？模拟标准库里面的Option枚举的意义。这里的枚举与C语言是有区别的。
}
