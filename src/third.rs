use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn append(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
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

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|node| &**node),
        }
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

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(node) = Rc::try_unwrap(node) {
                head = node.next;
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use testdrop::TestDrop;

    #[test]
    fn test_empty_on_creation() {
        let list: List<i32> = List::new();
        assert!(list.is_empty());
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn test_sizes() {
        let mut list = List::new();
        list = list.append(1);
        list = list.append(2);
        list = list.append(3);
        list = list.append(4);
        assert_eq!(list.size(), 4);
        assert_eq!(list.tail().size(), 3);
        list = list.tail().tail().tail().tail();
        assert!(list.is_empty());
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn test_iter() {
        let list = List::new().append(3).append(2).append(1);
        let iter = list.iter();
        for (i, &val) in iter.enumerate() {
            assert_eq!(i + 1, val);
        }
    }

    #[test]
    fn test_list_drop() {
        let testdrop = TestDrop::new();
        let list = List::new().append(testdrop.new_item());
        let list2 = list.append(testdrop.new_item());
        assert_eq!(testdrop.num_tracked_items(), 2);
        drop(list2);
        assert_eq!(testdrop.num_dropped_items(), 1);
    }
}
