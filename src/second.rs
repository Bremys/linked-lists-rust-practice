pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        self.head = Some(Box::new(Node {
            elem: elem,
            next: self.head.take(),
        }))
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
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

pub struct IntoIter<T>(List<T>);

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

impl<T> Drop for List<T> {
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
        let list: List<i32> = List::new();
        assert!(list.is_empty());
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn test_sizes() {
        let mut list: List<i32> = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        assert_eq!(list.size(), 4);
        list.pop();
        assert_eq!(list.size(), 3);
        list.pop();
        list.pop();
        list.pop();
        assert!(list.is_empty());
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn test_pop_empty() {
        let mut empty_list: List<i32> = List::new();
        assert_eq!(empty_list.pop(), None)
    }

    #[test]
    fn test_push_pops() {
        let mut list = List::new();
        list.push(32);
        list.push(40);
        assert_eq!(list.pop(), Some(40));
        list.push(20);
        assert_eq!(list.pop(), Some(20));
        assert_eq!(list.pop(), Some(32));
        assert_eq!(list.pop(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_peek() {
        let mut list = List::new();
        let first = "1";
        let second = "2";
        list.push(first);
        list.push(second);
        assert_eq!(list.peek(), Some(&second));
        list.pop();
        assert_eq!(list.peek(), Some(&first));
        list.pop();
        assert_eq!(list.peek(), None);
    }

    #[test]
    fn test_peek_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        let result = list.peek_mut();
        result.map(|val| *val = 3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_intoiter() {
        let mut list = List::new();
        list.push(3);
        list.push(2);
        list.push(1);
        let iter = list.into_iter();
        for (i, val) in iter.enumerate() {
            assert_eq!(i + 1, val);
        }
    }

    #[test]
    fn test_iter() {
        let mut list = List::new();
        list.push(3);
        list.push(2);
        list.push(1);
        let iter = list.iter();
        for (i, &val) in iter.enumerate() {
            assert_eq!(i + 1, val);
        }

        assert_eq!(list.pop(), Some(1));
        let iter = list.iter();

        for (i, &val) in iter.enumerate() {
            assert_eq!(i + 2, val);
        }
    }

    #[test]
    fn test_iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);
        let iter = list.iter_mut();
        for val in iter {
            *val *= 2;
        }
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(2));
    }
}
