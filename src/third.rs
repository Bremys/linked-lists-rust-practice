use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    size: u32,
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
                size: self.size() + 1,
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

    pub fn size(&self) -> u32 {
        self.head.as_ref().map(|node| node.size).unwrap_or_default()
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
        let result = self.next.map(|node| &node.elem);
        self.next = self
            .next
            .and_then(|node| node.next.as_ref().map(|node| &**node));
        result
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut curr_link = self.head.take();
        while let Some(node_ref) = curr_link {
            match Rc::try_unwrap(node_ref) {
                Ok(mut node) => curr_link = node.next,
                Err(_) => break,
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
        let list = List::new()
            .append(testdrop.new_item())
            .append(testdrop.new_item());
        let list2 = list.append(testdrop.new_item());
        let list3 = list.append(testdrop.new_item());
        assert_eq!(testdrop.num_tracked_items(), 4);
        drop(list);
        // Data is still referenced by list2 and list3 so nothing is dropped
        assert_eq!(testdrop.num_dropped_items(), 0);
        drop(list2);
        // Data exclusive to list2 is dropped while all else is still referenced by list3
        assert_eq!(testdrop.num_dropped_items(), 1);
        drop(list3);
        // All data is dropped
        assert_eq!(testdrop.num_dropped_items(), 4);
    }

    #[test]
    fn test_tail_on_empty() {
        let list: List<i32> = List::new().tail();
        assert!(list.is_empty());
        assert_eq!(list.size(), 0);
    }
}
