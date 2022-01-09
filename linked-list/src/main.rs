// Safe LinkedList
// Issues:
// - cyclic dependency;
// - memory leak;

use std::cell::RefCell;
use std::rc::Rc;

pub struct LinkedList<T> {
    head: Rc<RefCell<Node<T>>>,
    tail: Rc<RefCell<Node<T>>>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Default)]
struct Node<T> {
    elem: Option<T>,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T>
where
    T: Default + Clone,
{
    fn new(elem: T) -> Self {
        Self {
            elem: Some(elem),
            prev: None,
            next: None,
        }
    }
}

impl<T> LinkedList<T>
where
    T: Default + Clone,
{
    pub fn new() -> Self {
        let head = Rc::new(RefCell::new(Node::<T>::default()));
        let tail = Rc::new(RefCell::new(Node::<T>::default()));

        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(head.clone());
        Self { head, tail }
    }

    pub fn push(&mut self, elem: T) {
        let node = Rc::new(RefCell::new(Node::new(elem)));
        node.borrow_mut().next = Some(self.tail.clone());
        node.borrow_mut().prev = self.tail.borrow_mut().prev.clone();

        let prev = self.tail.borrow().prev.clone().unwrap();
        prev.borrow_mut().next = Some(node.clone());

        self.tail.borrow_mut().prev = Some(node.clone());
    }

    pub fn pop(&mut self) -> Option<T> {
        let node = self.head.borrow().next.clone().unwrap();
        let e = node.borrow().elem.clone();
        match e {
            Some(v) => {
                self.head.borrow_mut().next = node.borrow().next.clone();

                node.borrow_mut().prev = None;
                node.borrow_mut().next = None;

                Some(v)
            }
            None => None,
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T>
where
    T: Default + Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn basics() {
        let mut list = LinkedList::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn iter_should_work() {
        let mut list = LinkedList::new();
        list.push(1);list.push(2);list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);

    }
}

fn main() {
    println!("hello world")
}
