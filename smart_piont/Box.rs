// Box 是堆上分配的内存，通过 Box::new() 会创建一个堆空间并返回一个指向堆空间的指针
// 可以理解为一个关键字，只是用Box产生的指针，不需要手动释放了。
// 注意理解，就可以了，其实没有想想的那么困难的。

//想想也是，很多都是在栈上分配，这里给我们提供了一个借口，在堆上分配。

fn main() {
    println!("------------------begin!----");
    let piont_box = Box::new(12);



    println!("------------------end!----");

}
