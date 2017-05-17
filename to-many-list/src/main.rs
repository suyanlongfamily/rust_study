//cargo new --lib [project_name]
// 这个文件用于测试。
// 测试文件。

mod seconds;
use seconds::List;
// use seconds::Iter;
#[cfg(test)]
mod test {
    // use super::List;

    // #[test]
    // fn basics() {
    //     let mut list = List::new();

    //     // Check empty list behaves right
    //     assert_eq!(list.pop(), None);

    //     // Populate list
    //     list.push(1);
    //     list.push(2);
    //     list.push(3);

    //     // Check normal removal
    //     assert_eq!(list.pop(), Some(3));
    //     assert_eq!(list.pop(), Some(2));

    //     // Push some more just to make sure nothing's corrupted
    //     list.push(4);
    //     list.push(5);

    //     // Check normal removal
    //     assert_eq!(list.pop(), Some(5));
    //     assert_eq!(list.pop(), Some(4));

    //     // Check exhaustion
    //     assert_eq!(list.pop(), Some(1));
    //     assert_eq!(list.pop(), None);
    // }

    // #[test]
    // fn peek() {
    //     let mut list = List::new();
    //     assert_eq!(list.peek(), None);
    //     assert_eq!(list.peek_mut(), None);
    //     list.push(1);
    //     list.push(2);
    //     list.push(3);

    //     assert_eq!(list.peek(), Some(&3));
    //     assert_eq!(list.peek_mut(), Some(&mut 3));
    // }

    #[test]
    fn into_iter1() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter(); //这种引用是不是已经转移所有权了。
        assert_eq!(iter.next(), Some(3));
        list.pop(); //是否变换了所有权？？？
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
    }

    // #[test]
    // fn iter() {
    //     let mut list = List::new();
    //     list.push(1);
    //     list.push(2);
    //     list.push(3);

    //     let mut iter = list.iter();
    //     assert_eq!(iter.next(), Some(&3));
    //     assert_eq!(iter.next(), Some(&2));
    //     assert_eq!(iter.next(), Some(&1));
    // }

    // #[test]
    // fn iter_mut() {
    //     let mut list = List::new();
    //     list.push(1);
    //     list.push(2);
    //     list.push(3);

    //     let mut iter = list.iter_mut();
    //     assert_eq!(iter.next(), Some(&mut 3));
    //     assert_eq!(iter.next(), Some(&mut 2));
    //     assert_eq!(iter.next(), Some(&mut 1));
    // }
}


fn main() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.into_iter(); //这种引用是不是已经转移所有权了。
    // assert_eq!(iter.next(), Some(3));
    // list.pop(); //是否变换了所有权!!!!报错了！！！看到所有权转移到了迭代器里面了。
    // assert_eq!(iter.next(), Some(2));
    // assert_eq!(iter.next(), Some(1));

    //到底是 不可变引用能调用可变的成员函数，这个是错误的！！！！！！！ 想想权限问题，就明白了。
    //还是   可变的引用能调用不可变的函数。
    //关于这个成员函数能否调用，思考一下所有权的规则就知道了。！！！！！
    let iter_ref = &iter;
    // (*iter_ref).cloned();//引用不能喜欢一所有权，这个一定要明白了！！！！
    // let iii = *iter_ref;
    // iii.cloned();
}