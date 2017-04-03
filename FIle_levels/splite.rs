
// $ tree .
// .
// |-- my
// |   |-- inaccessible.rs
// |   |-- mod.rs
// |   `-- nested.rs
// `-- split.rs

// 文件分层
// 模块可以分配到文件/目录的层次结构中。代码拆开分到多个文件中

// split.rs
// 此声明将会查找名为 `my.rs` 或 `my/mod.rs` 的文件，并将该文件的内容插入到此作用域名为 `my` 的模块里面。
mod my;//导入

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}