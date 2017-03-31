use std::mem; //计算内存的包。
fn main() {
    //元组类型是自己组合的，通过()
    type Elment = (i32, String); //定义别名，通过type关键字。
    let e: Elment = (12, "12312".to_string()); //转化类型
    println!("elemt first {},and second {}", e.0, e.1); //注意元组对象的访问形式，通过索引访问。
    println!("{:?}", &e);    
    let (first, second) = e; //解构元组。
    println!("----first = {}and second = {}", first, second);

    // slice 类型 &[T] 
    let xs: [i32; 5] = [23, 4, 4, 23, 312];
    let ys: [i32; 500] = [0; 500];
    print!("ys arry length{}", ys.len()); //数组有成员函数len(),看来和C 语言室友区别的。在堆中分配。
    println!("array size:{}", mem::size_of_val(&xs));//5 * 4 = 20
    println!("array size:{}", mem::size_of_val(&ys));//500 * 4 = 2000
    


    
}

