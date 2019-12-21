use std::cell::RefCell;
use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem: elem,
            next: None,
            prev: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(node) => {
                node.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(node);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            match node.borrow_mut().next.take() {
                Some(next_node) => {
                    next_node.borrow_mut().prev.take();
                    self.head = Some(next_node);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(node).ok().unwrap().into_inner().elem
        })
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = Node::new(elem);
        match self.tail.take() {
            Some(node) => {
                node.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(node);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|node| {
            match node.borrow_mut().prev.take() {
                Some(next_node) => {
                    next_node.borrow_mut().next.take();
                    self.tail = Some(next_node);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(node).ok().unwrap().into_inner().elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use testdrop::TestDrop;

    #[test]
    fn test_push_pop_front() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_push_pop_back() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
        assert_eq!(list.pop_front(), None);
    }

    #[test]
    fn test_push_front_pop_back() {
        let mut list = List::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_push_tail_pop_front() {
        let mut list = List::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_list_drop() {
        let testdrop = TestDrop::new();
        let mut list = List::new();
        list.push_front(testdrop.new_item());
        list.push_front(testdrop.new_item());
        list.push_front(testdrop.new_item());
        list.push_back(testdrop.new_item());
        list.push_back(testdrop.new_item());
        list.push_back(testdrop.new_item());
        assert_eq!(6, testdrop.num_tracked_items());
        drop(list);
        assert_eq!(6, testdrop.num_dropped_items());
    }
}
