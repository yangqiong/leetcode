// https://leetcode.cn/problems/symmetric-tree/
// 给你一个二叉树的根节点 root ， 检查它是否轴对称。

struct Solution {}
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
//
impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(root) => Solution::is_symmetric_nodes(
                root.borrow().left.as_ref(),
                root.borrow().right.as_ref(),
            ),
        }
    }

    fn is_symmetric_nodes(
        left: Option<&Rc<RefCell<TreeNode>>>,
        right: Option<&Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (None, None) => true,
            (None, Some(_)) => false,
            (Some(_), None) => false,
            (Some(left), Some(right)) => {
                if left.borrow().val != right.borrow().val {
                    return false;
                } else {
                    return Solution::is_symmetric_nodes(
                        left.borrow().left.as_ref(),
                        right.borrow().right.as_ref(),
                    ) && Solution::is_symmetric_nodes(
                        left.borrow().right.as_ref(),
                        right.borrow().left.as_ref(),
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
