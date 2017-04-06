// 前面我们了解到，当被模式匹配命中的时候，未实现Copy的类型会被默认的move掉，因此，原owner就不再持有其所有权。
// 但是有些时候，我们只想要从中拿到一个变量的（可变）引用，而不想将其move出作用域，怎么做呢？答：用ref或者ref mut。

// 解引用使用 *
// 解构使用 &，ref， 和 ref mut

fn main() {
    let ref_val = 23;
    let ref ref_values = ref_val; //引用也可以使用ref关键字定义呀！！！
    let ref_oth = &ref_val;

    // let mut count = 0;
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {:?}!", state);
    // } else {
    //     count += 1;
    // }

    // 全部都是 `Option<i32>` 类型
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;
    // let iii = Some(22);
    let jjj = Some(222f32);
    if let Some(iii) = jjj {
        // 这个解构，主要是let表达式的作用，
        println!("---555--");
    } else {
        println!("-----");
    }

    // `if let` 结构解读：若 `let` 将 `number` 解构成 `Some(i)`，则执行语句块（`{}`）
    // ` = ` 不需要在看成赋值了，变成匹配意义了。
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // 如果要指明失败情形，就使用 else：
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // 解构失败。换到失败情形（Change to the failure case）。
        println!("Didn't match a number. Let's go with a letter!");
    };

    // 提供一个改变的失败条件（Provide an altered failing condition）。
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
        // 解构失败。执行 `else if` 条件来判断轮到的失败分支是否需要执行
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // 条件执行错误。这是默认的分支：
        println!("I don't like letters. Let's go with an emoticon :)!");
    };
}