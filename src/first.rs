use std::mem;

pub struct List<T> {
    head: Link<T>,
}

enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: T) {
        self.head = Link::More(Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        }))
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        match &self.head {
            Link::Empty => true,
            Link::More(_) => false,
        }
    }

    pub fn size(&self) -> i32 {
        let mut result = 0;
        let mut curr = &self.head;
        while let Link::More(node) = curr {
            curr = &node.next;
            result += 1;
        }
        result
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_link) = curr_link {
            curr_link = mem::replace(&mut boxed_link.next, Link::Empty);
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
        let mut list = List::new();
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
}
