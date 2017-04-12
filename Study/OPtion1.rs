/*
 * @CreateTime: Apr 7, 2017 10:48 AM 
 * @Author: undefined 
 * @Contact: undefined 
 * @Last Modified By: undefined
 * @Last Modified Time: Apr 7, 2017 11:28 AM
 * @Description: Modify Here, Please  
 */




// Option 查看源代码，这个是内置系统类型啊，不是std或者第三方类型。类是i32\f32等等这些类型一样。
// Option 是Rust的系统类型，用来表示值不存在的可能，这在编程中是一个好的实践，它强制Rust检测和处理值不存在的情况。
//系统也给它实现了更多的成员函数。

fn find(findstr: &str, needle: char) -> Option<usize> {
    for (offset, c) in findstr.char_indices() {
        if c==needle{
            return Some(offset);// 从这里可以看到把值通过Option类型包装一下返回。
        }
    }
    return None;
}
 fn main() {
     let file_name = "foobar.rs";
     match find(file_name,'.'){
         None => println!("none "),//这里其实是匹配的问题，rust编译器，自动给None、Some元组进行标识表示。
         Some(i) => println!("values file name = {}",&file_name[0..i]),
         
     }
 }



