fn main() {
    //定义数组
    let mut arrtobj:[i32;3] = [0;3];
    arrtobj[1] = 1;
    arrtobj[2] = 2;
    assert_eq!([1,2],&arrtobj[1..]);//对数组的引用。
    
    //数组的引用才有迭代器。
    for pat in &arrtobj {
        println!("{}---",pat);
    }
    
    
}