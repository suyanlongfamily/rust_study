

// 'Deref'强制转换
// 标准库提供了一个特殊的特征，Deref。它通常用于重载 * ，取消引用运算符：
// http://wiki.jikexueyuan.com/project/rust/deref-coercions.html
use std::ops::Deref;
use std::ops::Drop;

#[derive(Debug)]
struct DeferStudy<T> {
    values: T,
}

impl<T> DeferStudy<T> {
    fn ret(&self) -> &T {
        &self.values
    }
}

///对解引用进行重载。
impl<T> Deref for DeferStudy<T> {
    type Target = T;
    fn deref(&self) -> &T {
        println!("------");
        &self.values
    }
}

impl<T> Drop for DeferStudy<T> {
    fn drop(&mut self) {
        println!("---自定义drop函数，在对象生命周期结束时，自动执行，做些清理工作---");
    }
}


// #[stable(feature = "rust1", since = "1.0.0")]
// impl<'a, T: ?Sized> Deref for &'a T {
//     type Target = T;
// //代表的意思是什么呢？？？？？？？ 
// // T 元素实现固定的类型，并且实现 Deref trait 特性。 &'a T 本省就是一个引用，
//     fn deref(&self) -> &T {
//         *self//返回一个引用。
//     }
// }



fn main() {
    println!("-----------------begin--------");
    {
        let mut def = DeferStudy { values: 'a' }; //这个是分配在栈里面。
        assert_eq!('a', (&&&&&&&&&def).values); //这个是对拥有者进行操作了。不代表是指针，已经进行重载了。相当于一个函数调用！！！！记住啊！！！！
        (&&&&def).ret();
        //引用插入&&&都是一个&有效。
        let ref_def = &def;
        ref_def.values;
    }

    // println!("-----{}---", *def); //相当于一个函数调用！！！！记住啊！！！！
    // 这用于编写自定义指针类型。然而，有一个与 Deref 相关的语言特征：‘deref 强制转换’。
    // 规则是这样的：如果你有一个类型 U，它实现 Deref<Target=T>，&U 的值自动强制转换为 &T。


    // fn foo(s: &str) {
    //     // borrow a string for a second
    //     println!("----foo---={:?}-----",s);
    // }

    // // String implements Deref<Target=str>
    // let owned = "Hello".to_string();

    // // therefore, this works: 即 &U -> &T强制转换！！！！
    // foo(&owned);
//---------------------------------------------------------------------------------------------------
    // fn foo(s: &[i32]) {
    //     // borrow a slice for a second
    // }
    // // Vec<T> implements Deref<Target=[T]>
    // let owned = vec![1, 2, 3];
    // foo(&owned);

    // struct Foo;
    // impl Foo {
    //     fn foo(&self) {
    //         println!("Foo");
    //     }
    // }
    // let f = Foo;
    // f.foo();//怎么怎么换的？？？？？怎么理解？？？

    println!("-----------------end--------");
}