fn main() {
    // 将 `optional` 设为 `Option<i32>` 类型
    let mut optional = Some(0);

    // 分析：当 `let` 将 `optional` 解构成 `Some(i)` 时，就
    // 执行语句块（`{}`）。否则中断退出（`break`）。
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;//退出条件 值为None
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);//while 迭代器更新。
        }
        // ^ 使用的缩进更少，并且不用显式地处理失败情况。
    }
    // ^ `if let` 有额外可选的 `else`/`else if` 分句，
    // 而 `while let` 没有。
}