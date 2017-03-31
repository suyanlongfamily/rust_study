// 原生类型
// Rust内置的原生类型 (primitive types) 有以下几类：

// 布尔类型：有两个值true和false。
// 字符类型：表示单个Unicode字符，存储为4个字节。
// 数值类型：分为有符号整数 (i8, i16, i32, i64, isize)、 无符号整数 (u8, u16, u32, u64, usize) 以及浮点数 (f32, f64)。
// 字符串类型：最底层的是不定长类型str，更常用的是字符串切片&str和堆分配字符串String， 其中字符串切片是静态分配的，有固定的大小，并且不可变，而堆分配字符串是可变的。
// 数组：具有固定大小，并且元素都是同种类型，可表示为[T; N]。
// 切片：引用一个数组的部分数据并且不需要拷贝，可表示为&[T]。
// 元组：具有固定大小的有序列表，每个元素都有自己的类型，通过解构或者索引来获得每个元素的值。
// 指针：最底层的是裸指针*const T和*mut T，但解引用它们是不安全的，必须放到unsafe块里。
// 函数：具有函数类型的变量实质上是一个函数指针。
// 元类型：即()，其唯一的值也是()。

// 有几点是需要特别注意的：

// 数值类型可以使用_分隔符来增加可读性。
// Rust还支持单字节字符b'H'以及单字节字符串b"Hello"，仅限制于ASCII字符。 此外，还可以使用r#"..."#标记来表示原始字符串，不需要对特殊字符进行转义。
// 使用&符号将String类型转换成&str类型很廉价， 但是使用to_string()方法将&str转换到String类型涉及到分配内存， 除非很有必要否则不要这么做。
// 数组的长度是不可变的，动态的数组称为Vec (vector)，可以使用宏vec!创建。
// 元组可以使用==和!=运算符来判断是否相同。
// 不多于32个元素的数组和不多于12个元素的元组在值传递时是自动复制的。
// Rust不提供原生类型之间的隐式转换，只能使用as关键字显式转换。
// 可以使用type关键字定义某个类型的别名，并且应该采用驼峰命名法。


fn main() {
    let flag = true;
    if flag {
        println!("bool flag = true");
    } else {
        println!("bool flag = false");
    }
    let char_val = 'a';
    let char_valoth = 23;
    //类型转换是怎么转换的？
    // assert!(char_val, char_valoth);
     if char_val == 'a' {
        println!("char_val == 'a'");    
    }
    
    // str 、&str 与 String的区别????
    //&str 其实就是char * const 类型 编译器就分配好的字符串，在rust里面显示类型是： &'static str
    //即，你在代码里写的，所有的用""包裹起来的字符串，都被声明成了一个不可变，静态的字符串。
    // String重新变成&str呢？ 答：用 &* 符号   ||  let mut y:String = x.to_string(); &'static str 使用 to_string()函数
    //
    let str_val = "hello_world";
    let string_val:String = str_val.to_string();
    

    //数组
    let  arry = [1,2,3,4,5,6,7];
    let middle = &[1..4];//获取前四个通过引用。
    let mut ten_zeros:[i64;10] = [0;10];//定义并初始化。
    // 怎么遍历一个数组？
    for i in &arry {//又是只能是应用形式。
        println!("----{}",i);
    }   

    //元组定义
    // &str 这种形式感觉就是C、C++里面的const char* 这种类型呢？ 
    let tulp_val:(i32,&str) = (12,"123");
    //元组访问，需要用到下标索引0，1，2，...
    println!("print tulp elment {}",tulp_val.0);

    //raw pionter 
    //看到这里，可以真正明白rust的引用& 其实 *，是吧！！！
    //通过调试，观察 &val， 就是代表val变量的指针（地址），val一定要存在！才能看到。这下明白rust所有权、引用、可变引用的意思了吧。
    let x = 5;
    let raw = &x as *const i32;//不可变指针
    let pointer_at = unsafe{*raw};//unsafe 关键字包裹
    println!("pointer_at values = {}",pointer_at);//这里明白了吧，同样也可以调试一下。看看各个变量的值。

    //函数类型
    //函数里可以定义函数，有一个类C语言的区别！！！！，不过想想也是，既然吸收百家之长，肯定要有类js里面的特性啊！
    fn foo(x:i32)->i32{x}
    let bar:fn(i32)->i32 = foo;
    println!("foo == bar return values = {},{}",bar(32),foo(32));



}