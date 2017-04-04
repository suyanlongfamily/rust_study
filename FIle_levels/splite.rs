
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
// 编译后，生成一个可执行文件。
// 细心的朋友会发现，aaa.rs 中，没有使用 mod xxx {} 这样包裹起来，
// 是因为 mod xxx; 相当于把 xxx.rs 文件用 mod xxx {} 包裹起来了。初学者往往会多加一层，请注意。
mod my; //导入到my模块的文件是mod.rs文件的内容，不是当前文件的内容。区别一下。
// use my;use 不能导入根模块。必须使用mod导入根模块，在使用use导入其子模块。
// use nested;这个模块是my子模块，my都没有还么有导入到当前文件呢。这个也验证模块没有导入时，使用use导入子模块不行！！
mod test;//不能使用::符号。

fn function() {
    println!("called `function()`");
    print();
}

fn main() {
    my::function();
    // use my::nested::function;//不能在同一个作用域存在两个function。
    function();
    {
        use my::nested::function;
        function();
    }
    my::indirect_access();
    my::nested::function();
}


// 多文件模块的层级关系

// Rust 的模块支持层级结构，但这种层级结构本身与文件系统目录的层级结构是解耦的。

// mod xxx; 这个 xxx 不能包含 :: 号。也即在这个表达形式中，是没法引用多层结构下的模块的。也即，你不可能直接使用 mod a::b::c::d; 的形式来引用 a/b/c/d.rs 这个模块。

// 那么，Rust 的多层模块遵循如下两条规则：

// mod xxx; 默认优先查找，同级目录下的 xxx.rs 文件；
// 如果 xxx.rs 不存在，则查找 xxx/mod.rs 文件，即 xxx 目录下的 mod.rs 文件。
// 上述两种情况，加载成模块后，效果是相同的。Rust 就凭这两条规则，通过迭代使用，结合 pub 关键字，实现了对深层目录下模块的加载；