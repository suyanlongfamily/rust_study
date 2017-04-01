fn main() {
    
    let color = "gree";
    let i_val = 2;
    // i_val = 12;
    fn fun1(i: i32) -> i32 {
        i + 1
    }
    //以下调用多次的结果是一样的，需要注意了。
    println!("{}", fun1(i_val));
    println!("{}", fun1(i_val));
    println!("{}", fun1(i_val));
    println!("{}", fun1(i_val));
    println!("{}", fun1(i_val));
    println!("{}", fun1(i_val));
    println!("-------");
    let colsure_i = |i: i32| {i + 1};
    //下面调用结果也都是一样的。为什么呢？？
    println!("{}", colsure_i(i_val));
    println!("{}", colsure_i(i_val));
    println!("{}", colsure_i(i_val));
    println!("-------");
    //下面调用结果也都是一样的。为什么呢？？
    println!("{}", extern_fun(i_val));
    println!("{}", extern_fun(i_val));
    println!("{}", extern_fun(i_val));
    
    let mut j_val = 1;
    let mut closure_j = |i| i + 1;
    println!("closure_j---{}", closure_j(j_val));
    println!("closure_j---{}", closure_j(j_val));

    //以上都是值copy的形式，原值都是不变的，所以，不管是内部函数还是外部函数，亦或者闭包都是一样的。
    //只是否是拷贝类型、是否引用、是否可变、是否所有权转移、是否生命周期、是否作用域。需要注意的。
    {
        let  mut  closure_nee = || {
            // 闭包是存储上下文环境，同理，如果想对上文进行修改，需要加上mut 关键，
            j_val += 1;//对上下文进行了修改，所以加mut关键字。
            println!("closure_nee ---{}", j_val);
        };
        closure_nee();
        // println!("----{}",j_val);在这当前作用域内，不能使用了，因为闭包一直存在了，转移了所使用变量的所有权。
        closure_nee(); //这里可以看出来，闭包作用是使用下文环境（变量），或者提供上下文环境变量，通过所有权的转移。
        closure_nee();
        // println!("----{}",j_val);
        // let reboo = &mut j_val; 在闭包生命周期内，根据所有权的规则，原对象都无法再使用了。
    } //添加块级作用域就可以了。后面的才能使用闭包使用的值，
    println!("----{}", j_val);     
    
    {
        //使用内部函数
        fn fun_j() {
            // j_val += 1;//内部函数无法使用上下文环境变量。
            println!("closure_nee ---");
        }
        fun_j();
        fun_j();        
    }
    println!("----{}", j_val); 
    
    
    println!("large seperate -------------------------------------------------");
    //看看引用类型
    
    //  fn function() {
    //     println!("--{}", &color)
    // };
    // fn function() {
    //     println!("--{}", &color)
    // };
    // function();
    // function();
    let print_color = || println!("--{}", color);
    print_color();
    print_color();     
    
}

fn extern_fun(i: i32) -> i32 {    
    i + 1
}
