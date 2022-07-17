#[derive(Debug, Clone)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32, next: Option<Box<Node>>) -> Self {
        Node { val, next }
    }
}

#[derive(Debug)]
struct MyLinkedList {
    head: Option<Box<Node>>,
    count: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList {
            head: None,
            count: 0,
        }
    }

    fn _get(node: Option<&Box<Node>>, index: i32) -> i32 {
        if let Some(n) = node {
            if index == 0 {
                return n.val;
            } else {
                Self::_get(n.next.as_ref(), index - 1)
            }
        } else {
            -1
        }
    }

    fn get(&self, index: i32) -> i32 {
        Self::_get(self.head.as_ref(), index)
    }

    fn add_at_head(&mut self, val: i32) {
        self.head = Some(Box::new(Node::new(val, self.head.take())));
        self.count += 1;
    }

    fn _add_at_tail(node: Option<&mut Box<Node>>, val: i32) {
        if let Some(n) = node {
            if n.next.is_some() {
                Self::_add_at_tail(n.next.as_mut(), val)
            } else {
                n.next = Some(Box::new(Node::new(val, None)))
            }
        }
    }

    fn add_at_tail(&mut self, val: i32) {
        if self.head.is_some() {
            Self::_add_at_tail(self.head.as_mut(), val);
        } else {
            self.head = Some(Box::new(Node::new(val, None)));
        }
    }

    fn _add_at_index(node: Option<&mut Box<Node>>, index: i32, val: i32) {
        if let Some(n) = node {
            if n.next.is_some() {
                if index == 0 {
                    let new_node = Some(Box::new(Node::new(val, n.next.take())));
                    n.next = new_node;
                } else {
                    Self::_add_at_index(n.next.as_mut(), index - 1, val)
                }
            } else {
                if index == 0 {
                    n.next = Some(Box::new(Node::new(val, n.next.take())));
                }
            }
        }
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        let mut new_node = Node::new(val, None);
        if index <= 0 {
            self.add_at_head(val);
        } else {
            Self::_add_at_index(self.head.as_mut(), index - 1, val);
        }
    }

    fn _delete_at_index(node: Option<&mut Box<Node>>, index: i32) {
        if let Some(n) = node {
            if let Some(m) = n.next.as_mut() {
                if index == 0 {
                    n.next = m.next.take();
                } else {
                    Self::_delete_at_index(n.next.as_mut(), index - 1)
                }
            }
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if let Some(n) = self.head.as_mut() {
            if index == 0 {
                self.head = n.next.take();
            } else {
                Self::_delete_at_index(self.head.as_mut(), index - 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_e707() {
        let mut obj = MyLinkedList::new();
        obj.add_at_head(1);
        obj.add_at_tail(3);
        println!("{:?}", obj);
        obj.add_at_index(1, 2);
        println!("{:?}", obj);
        assert_eq!(obj.get(1), 2);
        obj.delete_at_index(1);
        assert_eq!(obj.get(1), 3);

        let mut obj = MyLinkedList::new();
        obj.add_at_index(1, 0);

        assert_eq!(obj.get(1), -1);

        let mut obj = MyLinkedList::new();
        obj.add_at_head(7);
        obj.add_at_head(2);
        obj.add_at_head(1);
        obj.add_at_index(3, 0);
        obj.delete_at_index(2);
        obj.add_at_head(6);
        obj.add_at_tail(4);
        assert_eq!(obj.get(4), 4);
    }
}
