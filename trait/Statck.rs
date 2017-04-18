//这里定义个栈Stack
// 注意定义数据结构，以及组织方式，这个还是很重要的。
//take：消耗、拿走、取、花费---注意英文单词的意义，代表使用方法。
// 注意mut self 、self 、&self、&mut self 之间的转化，一般都是通过自带的实现函数转化的。这一点很重要的。
#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

//主要用于调试,用debug trait
#[derive(Debug)]
struct Statck<T> {
    head: Option<Box<Node<T>>>,
    size: i32,
}

impl<T> Statck<T> {
    // create object
    fn new() -> Statck<T> {
        Statck {
            head: None,
            size: 0,
        }
    }

    fn size(&self) -> i32 {
        self.size
    }

    //返回一个用Option包装好的值，
    fn pop(&mut self) -> Option<T> {
        // unwrap()这个函数的使用规则冲突了。只能是所有者才可以调用，注意被调用函数的主体是什么样子的。
        // let mut box_node = self.head.take().unwrap();
        // let mut node = *box_node;
        // self.head = node.next; //指向头部
        // Some(node.val)
        //----------------------------------以上是学习C++语言的思维习惯。
        // 看到有Option，要里面想到match，或者if let 去结构它。注意类型的匹配，才是主要的。
        // 命名规则：类型_值名
        // 函数的链式调用，类型转换，这些都很重要的。
        // 多自己写一些代码，熟悉一下rust的编程习惯，
        let val = self.head.take();
        match val {
            None => None,
            Some(mut box_val) => {
                self.head = box_val.next.take();
                self.size -= 1;
                Some(box_val.val)
            }
        }
    }
    //把顶部节点置换指向。
    fn push(&mut self, val: T) {
        let new_node = Box::new(Node {
                                    val: val,
                                    next: self.head.take(),
                                });
        self.head = Some(new_node);
        self.size += 1;
    }
}


fn main() {
    println!("----------------begin----------------");

    let mut statck = Statck::new();
    statck.push(12);
    statck.push(12);
    println!("-----------size = {}", statck.size());
    statck.push(12);
    println!("-----------size = {}", statck.size());
    statck.push(12);
    println!("-----------size = {}", statck.size());
    statck.push(12);
    println!("-----------size = {}", statck.size());

    statck.pop();
    statck.pop();
    println!("------pop()-----size = {}", statck.size());

    println!("----------------end----------------");
}
