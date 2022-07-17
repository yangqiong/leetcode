// https://leetcode.cn/problems/min-stack/
// 设计一个支持 push ，pop ，top 操作，并能在常数时间内检索到最小元素的栈。

#[derive(Debug)]
struct MinStack {
    data: Vec<i32>,
    min: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack {
            data: vec![],
            min: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.data.push(val);
        if let Some(&min) = self.min.last() {
            if val <= min {
                self.min.push(val);
            }
        } else {
            self.min.push(val);
        }
    }

    fn pop(&mut self) {
        if let Some(val) = self.data.pop() {
            if let Some(&min) = self.min.last() {
                if val == min {
                    self.min.pop();
                }
            }
        }
    }

    fn top(&self) -> i32 {
        *self.data.last().unwrap()
    }

    fn get_min(&self) -> i32 {
        *self.min.last().unwrap()
    }
}
