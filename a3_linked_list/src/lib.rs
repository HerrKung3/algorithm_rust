//连接方式
type Link<T> = Option<Box<Node<T>>>;

//节点
struct  Node<T> {
    elem: T,
    next: Link<T>
} 

//链表
pub struct List<T> {
    size: usize,
    head: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { size: 0, head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, elem: T) {
        let node = Box::new(Node {elem, next: self.head.take()});
        self.head = Some(node);
        self.size += 1;
    }

    //.take()取出Some(Box<Node<T>>)或None，通常用于操作Option，取出后将其设置为None
    //.map()操作.take()返回值，如果是Some，则传入闭包
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.elem
        })
    }

    //返回不可变引用
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    //返回可变引用
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node|&mut node.elem)
    }

    //链表改变，成为迭代器
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    //链表不变，得到不可变迭代器
    pub fn iter(&self) -> Iter<T> {
        Iter{next: self.head.as_deref()}
    }

    //链表不变，得到可变迭代器
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {next: self.head.as_deref_mut()}
    }
}

pub struct IntoIter<T>(List<T>);

impl <T>Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>
}

impl <'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node|{
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>
}

impl <'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take();
        }
    }
}
