
use std::mem;
fn main() {
    // slice 类型 &[T] 
    let xs: [i32; 5] = [23, 4, 4, 23, 312];
    let ys: [i32; 500] = [0; 500];
    print!("ys arry length{}", ys.len()); //数组有成员函数len(),看来和C 语言有区别的。
    //数组是在堆中分配。
    println!("array size:{}", mem::size_of_val(&xs)); //5 * 4 = 20
    println!("array size:{}", mem::size_of_val(&ys)); //500 * 4 = 2000

    //slice 类型。
    anlayze_slice(&ys);
    anlayze_slice(&ys[1..4]);    
}

fn anlayze_slice(arg: &[i32]){
    println!("first element of the slice:{}",arg[0]);
    println!("the slice has {} elements",arg.len());
}