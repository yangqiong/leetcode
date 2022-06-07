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

    fn get(&self, index: i32) -> i32 {
        let mut i = 0;
        let mut node = self.head.as_ref();
        let mut result = -1;
        loop {
            if let Some(n) = node {
                if i == index {
                    result = n.val;
                    break;
                } else {
                    node = n.next.as_ref();
                    i += 1;
                }
            } else {
                break;
            }
        }
        result
    }

    fn add_at_head(&mut self, val: i32) {
        self.head = Some(Box::new(Node::new(val, self.head.take())));
        self.count += 1;
    }

    fn add_at_tail(&mut self, val: i32) {
        let tail = Node::new(val, None);
        let mut node = self.head.as_mut();
        loop {
            if let Some(mut n) = node {
                if n.next.is_some() {
                    node = n.next.as_mut()
                } else {
                    n.next = Some(Box::new(tail));
                    break;
                }
            } else {
                self.head = Some(Box::new(tail));
                break;
            }
        }
        self.count += 1;
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        let mut new_node = Node::new(val, None);
        if index <= 0 {
            self.add_at_head(val);
        } else if index <= self.count {
            let mut i = 1;
            let mut node = self.head.as_mut();
            loop {
                if let Some(mut n) = node {
                    if i == index {
                        new_node.next = n.next.take();
                        n.next = Some(Box::new(new_node));
                        break;
                    } else {
                        i += 1;
                        node = n.next.as_mut();
                    }
                } else {
                    break;
                }
            }
            self.count += 1;
        }
    }

    fn delete_at_index(&mut self, index: i32) {
        if index >= 0 && index < self.count {
            let mut i = 0;
            let mut node = self.head.as_mut();
            loop {
                if let Some(mut n) = node {
                    if index == 0 {
                        self.head = n.next.take();
                        break;
                    } else if let Some(m) = n.next.as_mut() {
                        if i == index - 1 {
                            n.next = m.next.take();
                            break;
                        } else {
                            node = n.next.as_mut();
                            i += 1;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            self.count -= 1;
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
        obj.add_at_index(1, 2);
        assert_eq!(obj.get(1), 2);
        obj.delete_at_index(1);
        assert_eq!(obj.get(1), 3);

        let mut obj = MyLinkedList::new();
        obj.add_at_index(1, 0);

        assert_eq!(obj.get(1), -1);

        let mut obj = MyLinkedList::new();
        obj.add_at_head(7);
        println!("{:?}", obj);
        obj.add_at_head(2);
        println!("{:?}", obj);
        obj.add_at_head(1);
        println!("{:?}", obj);
        obj.add_at_index(3, 0);
        println!("{:?}", obj);
        obj.delete_at_index(2);
        println!("{:?}", obj);
        obj.add_at_head(6);
        println!("{:?}", obj);
        obj.add_at_tail(4);
        println!("{:?}", obj);
        assert_eq!(obj.get(4), 4);
    }
}
