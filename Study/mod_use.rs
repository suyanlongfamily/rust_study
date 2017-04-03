// 将 `deeply::nested::function` 路径绑定到 `other_function`。
//

fn function() {
    println!("called `function()`");
}

// 这个是创建模块，隔离不同的代码功能，并且具有访问权限。同一个模块内没有访问权限。
mod deeply {
    pub mod nested {
        pub fn function() {
            println!("called `deeply::nested::function()`")
        }
    }
}

//use却是引入模块，并且重命名对引入的模块。
use deeply::nested::function as other_function;

fn main() {
    // 更容易访问 `deeply::nested::funcion`
    other_function();

    println!("Entering block");
    {
        // 这和 `use deeply::nested::function as function` 等价。
        // 此 `function()` 将覆盖掉的外部同名函数。
        use deeply::nested::function;
        function();

        // `use` 绑定拥有局部作用域。在这个例子中，`function()`的覆盖只限定在这个代码块中。
        println!("Leaving block");
    }
//外部的函数
    function();
}