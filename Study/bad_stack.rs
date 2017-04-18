
use std::mem;
// use std::Option;

pub struct List {
    head: Link,
}

//这个枚举的意思就是节点的值。要么是一个安全的指针，要么是一个空值。
// #[derive(Copy)]
enum Link {
    Empty, //节点值的取值
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
                                    elem: elem,
                                    next: mem::replace(&mut self.head, Link::Empty),
                                });
        self.head = Link::More(new_node);
    }

    // pub fn pop(&mut self) -> Option<i32> {
    //     // let result;
    //     // mem::replace 复制转移dest返回它，并把src所有权转移给dest。
    //     match mem::replace(&mut self.head, Link::Empty) {
    //         Link::Empty => None,
    //         Link::More(node) => {
    //             self.head = node.next;
    //             // Some(node.elem)
    //         }
    //     };
    //     Some(12)
    //     // result
    // }



    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                // self.head = (*node).next;//这些为什么不可以用呢？？？下面的就可以了。！！！！
                // Some((*node).elem)

                let node_obj = *node;//这个是解引用。
                self.head = node_obj.next;
                Some(node_obj.elem)
            }
        }
    }
}





 fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        // let ret = list.pop();
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
 }


fn main() {
    println!("------------");

    // use std::mem;
    let vvv = vec![12, 2, 4, 3];
    let vvvv = vvv; //可变与不可变都可以转移所有权。
    // print!("{:?}",vvv);

    let mut v: Vec<i32> = vec![1, 2];
    let old_v = mem::replace(&mut v, vec![3, 4, 5]);
    println!("{:?}", old_v);
    println!("{:?}", v);
    assert_eq!(2, old_v.len());
    assert_eq!(3, v.len());

    basics();
}
