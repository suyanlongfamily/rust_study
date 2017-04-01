// use Vec
// use std::vec::Vec;
//设置编译器参数，禁用提示，
// #![allow(unused_mut)]
# ! [allow(unused_variables)]
//#[warn(unused_variables)] on by default
# ! [allow(dead_code)]
fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 62, 12, 34, 6];
    //使用索引[]
    v[1] += v[2];
    v[2] = 100;
    for i in &v {
        println!("vector :{}", i);
    }
    //另一种创建Ver对象的形式
    let mut v_oth: Vec<i32> = std::vec::Vec::new(); //创建对象
    // let k = v_oth.get(1);
    // println!("{}",k);

    v_oth.push(123);
    v_oth.push(123);
    v_oth.push(123);
    v_oth.push(123);
    v_oth.push(123);
    let val = v_oth.pop();
    // println!("---{}", val);
    println!("---{}", v_oth.len());

    let mut contai_vec = vec![];
    addInfo(&mut contai_vec, 12);
    println!("13212");
println!("\{:?}",contai_vec);
}

fn fun(arg: i32) {}

fn addInfo(vec: &mut Vec<i32>, val: i32) {
    vec.push(val); //z
    (*vec).push(val);
    println!("---push success!");
}