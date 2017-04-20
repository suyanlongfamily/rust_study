pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,//变量
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}



impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
                                    elem: elem,
                                    next: self.head.take(),
                                });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head
            .take()
            .map(|node| {
                     let node = *node;
                     self.head = node.next;
                     node.elem
                 })
    }

    //返回一个option<&T> 对象
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self) //转移所有权了。
    }

    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}


//Iterator 里面封装了接口，
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next//有一个next成员对象。同样也有一个成员函数。
            .map(|node| {
                     self.next = node.next.as_ref().map(|node| &**node);
                     &node.elem
                 })
    }
}

//通过实现的方式。
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        //map 里面的闭包是要有符合类型的，
        self.next
            .take()
            .map(|node| {
                     self.next = node.next.as_mut().map(|node| &mut **node);
                     &mut node.elem
                 })
    }
}


fn main() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.into_iter(); //这种引用是不是已经转移所有权了。
    // list.push(123);已经被移动了在into_iter里面。。
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    // iter();
    iter1();
}

fn iter1() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.iter();//这里返回了一个迭代器。
    // iter.by_ref();
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&1));
}
