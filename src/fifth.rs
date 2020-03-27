use std::ptr;

pub struct Queue<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            elem: elem,
            next: None,
        });

        let raw_tail: *mut Node<T> = new_tail.as_mut();

        if self.tail.is_null() {
            self.head = Some(new_tail);
        } else {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        }

        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        let result = self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        });

        if self.head.is_none() {
            self.tail = ptr::null_mut();
        }

        result
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn size(&self) -> i32 {
        let mut result = 0;
        let mut curr = &self.head;
        while let Some(node) = curr {
            curr = &node.next;
            result += 1;
        }
        result
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }
}

pub struct IntoIter<T>(Queue<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        let mut curr_link = self.head.take();

        while let Some(mut boxed_link) = curr_link {
            curr_link = boxed_link.next.take();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_on_creation() {
        let queue: Queue<i32> = Queue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn test_sizes() {
        let mut queue: Queue<i32> = Queue::new();
        queue.push(1);
        queue.push(2);
        queue.push(3);
        queue.push(4);
        assert_eq!(queue.size(), 4);
        queue.pop();
        assert_eq!(queue.size(), 3);
        queue.pop();
        queue.pop();
        queue.pop();
        assert!(queue.is_empty());
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn test_pop_empty() {
        let mut empty_queue: Queue<i32> = Queue::new();
        assert_eq!(empty_queue.pop(), None)
    }

    #[test]
    fn test_push_pops() {
        let mut queue = Queue::new();
        queue.push(32);
        queue.push(40);
        assert_eq!(queue.pop(), Some(32));
        queue.push(20);
        assert_eq!(queue.pop(), Some(40));
        assert_eq!(queue.pop(), Some(20));
        assert_eq!(queue.pop(), None);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_peek() {
        let mut queue = Queue::new();
        let first = "1";
        let second = "2";
        queue.push(first);
        queue.push(second);
        assert_eq!(queue.peek(), Some(&first));
        queue.pop();
        assert_eq!(queue.peek(), Some(&second));
        queue.pop();
        assert_eq!(queue.peek(), None);
    }

    #[test]
    fn test_peek_mut() {
        let mut queue = Queue::new();
        queue.push(1);
        queue.push(2);
        let result = queue.peek_mut();
        result.map(|val| *val = 3);
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn test_intoiter() {
        let mut queue = Queue::new();
        queue.push(1);
        queue.push(2);
        queue.push(3);
        let iter = queue.into_iter();
        for (i, val) in iter.enumerate() {
            assert_eq!(i + 1, val);
        }
    }

    #[test]
    fn test_iter() {
        let mut queue = Queue::new();
        queue.push(1);
        queue.push(2);
        queue.push(3);
        let iter = queue.iter();
        for (i, &val) in iter.enumerate() {
            assert_eq!(i + 1, val);
        }

        assert_eq!(queue.pop(), Some(1));
        let iter = queue.iter();

        for (i, &val) in iter.enumerate() {
            assert_eq!(i + 2, val);
        }
    }

    #[test]
    fn test_iter_mut() {
        let mut queue = Queue::new();
        queue.push(3);
        queue.push(2);
        queue.push(1);
        let iter = queue.iter_mut();
        for val in iter {
            *val *= 2;
        }
        assert_eq!(queue.pop(), Some(6));
        assert_eq!(queue.pop(), Some(4));
        assert_eq!(queue.pop(), Some(2));
    }
}
