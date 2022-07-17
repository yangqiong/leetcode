// https://leetcode.cn/problems/minimum-depth-of-binary-tree/
// 给定一个二叉树，找出其最小深度。
// 最小深度是从根节点到最近叶子节点的最短路径上的节点数量。

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
use std::cmp::{max, min};
use std::rc::Rc;

struct Solution {}
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::min_depth_node(root.as_ref())
    }

    fn min_depth_node(node: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        match node {
            None => 0,
            Some(node) => {
                let left_depth = Self::min_depth_node(node.borrow().left.as_ref());
                let right_depth = Self::min_depth_node(node.borrow().right.as_ref());
                match (left_depth, right_depth) {
                    (0, _) => right_depth + 1,
                    (_, 0) => left_depth + 1,
                    _ => min(left_depth, right_depth) + 1,
                }
            }
        }
    }
}
