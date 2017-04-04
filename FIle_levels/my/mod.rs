// my/mod.rs
// 类似地，`mod inaccessible` 和 `mod nested` 将找到`inaccessible.rs` 和`nested.rs`文件，并在它们各自的模块中插入它们的内容。

mod inaccessible;//这个相当于，对inaccessible.rs文件内容进行inaccessible模块封装，并导入在mod.rs文件里面作为私有子模块
pub mod nested;//这个相当于，对inaccessible.rs文件内容进行inaccessible模块封装，并导入在mod.rs文件里面作为开放子模块

pub fn function() {
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    print!("called `my::indirect_access()`, that\n> ");

    private_function();
}

