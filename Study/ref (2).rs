#[derive(Clone, Copy,Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let c = 'Q';

    // 赋值语句中左边的 `ref` 关键字等价右边的 `&` 符号。
    let ref ref_c1 = c; //ref 可以这样用，同一目的不同角度
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = Point { x: 0, y: 0 };

    // 在解构一个结构体时 `ref` 同样有效。
    let _copy_of_x = {
        // `ref_to_x` 是一个指向 `point` 的 `x` 字段的引用。
        let Point {
            x: ref ref_to_x,
            y: _,
        } = point;

        // 返回一个 `point` 的 `x` 字段的拷贝。
        *ref_to_x //细细看这里，原来是使用块返回值。
    };

    println!("---_copy_of_x = {}", _copy_of_x);

    // `point` 的可变拷贝
    let mut mutable_point = point; //这个是copy类型，主要是里面的类型决定的。

    {
        // `ref` 可以结合 `mut` 来接受可变引用。
        let Point {
            x: _,
            y: ref mut mut_ref_to_y,
        } = mutable_point; //let这种解构模式也是很厉害了。

        // 通过可变引用来改变 `mutable_point` 的字段 `y`。
        *mut_ref_to_y = 1; //解引用改变实际指向的值。
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({}, {})",
             mutable_point.x,
             mutable_point.y);

    println!("--------{:?}------------", mutable_point);


    // 包含一个指针的可变元组
    let mut mutable_tuple = (Box::new(5u32), 3u32);
    {
        // 解构 `mutable_tuple` 来改变 `last` 的值。
        // 左边的ref mut 表示last 是一个可变的引用。
        let (_, ref mut last) = mutable_tuple;
        *last = 2u32;
        let ref mut val = 123u32;
        // last = val;//这种就不可以。
        *last = *val; //这种就可以

        {
            {
                let p_val: &i32 = &23; //符合情况，理解。
                let piont = &123; //右边的123 是一个临时对象。执行完这一句，就直接drop掉了。所以编译器报错。
                let llll = 123;
            }
            {
                let p_val: &mut i32 = &mut 23; //这个地方，mut就可以放到右边！！！！为什么？？？？？？？？
                *p_val = 1234;
            }
            {
                let p_val = &mut 23;
                *p_val = 122;
            }

            {
                let ref p_val = &23;
                // *p_val = 12
            }
            {
                let mut val = 23234;
                let mut val_oth = 23;
                let ref mut p_val = val; //这种就符合关键字定义了。ref 定义p_val 是一个引用，mut定义引用是的对象是可变的。
                // mut 关键字是修饰所有权的，不是修饰变量本身的。可变数据
                // *p_val = 12;
                // p_val = &mut val_oth; 出错。

                let ref mut pp_val = *p_val;
                *pp_val = 1123;
                // *p_val = 10000;//assignment to borrowed `*p_val` occurs here
                // p_val = &mut val_oth; //出错。对引用赋值。很重要！！！！！定义一个引用后，就不能再对引用本身赋值了！！！


                // let ppppp:mut i32 = 132;// mut 放到这里是不行的。！！！mut只能放到变量的左边。单独存在时。
                let mut ppppp: i32 = 132; // mut 放到这里可以。
                let ref mut piont;//这种定义是引用的对象是可变的。
                piont = &mut val_oth;
                // piont = &mut ppppp;//无法执行。再次确定，mut 只能用来修饰所有权，不能修饰引用。

                let asf = 132; //用于调试。
            }

            // p_val

        }
    }

    println!("tuple is {:?}", mutable_tuple);
}