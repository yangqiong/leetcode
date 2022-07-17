// https://leetcode.cn/problems/path-sum/
// 给你二叉树的根节点 root 和一个表示目标和的整数 targetSum 。判断该树中是否存在 根节点到叶子节点 的路径，这条路径上所有节点值相加等于目标和 targetSum 。如果存在，返回 true ；否则，返回 false 。

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
struct Solution {}

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            None => false,
            Some(root) => {
                let val = root.borrow().val;
                match (root.borrow().left.as_ref(), root.borrow().right.as_ref()) {
                    (None, None) => root.borrow().val == target_sum,
                    (None, Some(right)) => Self::has_path_sum_node(Some(right), target_sum - val),
                    (Some(left), None) => Self::has_path_sum_node(Some(left), target_sum - val),
                    (Some(left), Some(right)) => {
                        Self::has_path_sum_node(Some(left), target_sum - val)
                            || Self::has_path_sum_node(Some(right), target_sum - val)
                    }
                }
            }
        }
    }

    fn has_path_sum_node(node: Option<&Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        println!("{:?}", node);
        match node {
            None => false,
            Some(node) => {
                if node.borrow().left.is_none()
                    && node.borrow().right.is_none()
                    && target_sum == node.borrow().val
                {
                    return true;
                } else {
                    Self::has_path_sum_node(
                        node.borrow().left.as_ref(),
                        target_sum - node.borrow().val,
                    ) || Self::has_path_sum_node(
                        node.borrow().right.as_ref(),
                        target_sum - node.borrow().val,
                    )
                }
            }
        }
    }
}
