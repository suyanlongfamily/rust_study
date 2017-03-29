fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    // let v_bak = v;//把资源转移一个非修改的绑定上。如果使用新的&mut v_bak 就会出错。
    let mut v_bak = v; //move移动（把资源转移一个非修该的绑定上）。
    // for iter in &v {//使用移动的了值，会出错。
    //     println!("vector values:{}",iter);
    // }

    for iter in &v_bak {
        //使用移动的了值
        println!("vector values:{}", iter);
    }
    let mut var_oth = &mut v_bak;
    // var_oth = 12;
    fun(var_oth);
    var_oth.pop();
    for var in &(*var_oth) {
        println!("vector var values:{}", var);
    }
    // for var in v_bak {} //v_bak 借用出去的资源还没有收回，不能使用。
}

fn fun(vec: &mut Vec<i32>) {
    // let mut v1: Vec<i32> = Vec::new();
    vec.pop();
}