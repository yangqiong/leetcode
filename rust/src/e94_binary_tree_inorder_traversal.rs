// https://leetcode.cn/problems/binary-tree-inorder-traversal/
// 给定一个二叉树的根节点 root ，返回 它的 中序 遍历 。

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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return Vec::new();
        };

        let mut result: Vec<i32> = Vec::new();
        Solution::inorder_traversal_inner(&mut result, root.as_ref().unwrap());

        result
    }

    pub fn inorder_traversal_inner(values: &mut Vec<i32>, node: &Rc<RefCell<TreeNode>>) {
        let node = node.borrow();
        if let Some(ref left) = node.left {
            Solution::inorder_traversal_inner(values, left);
        }
        values.push(node.val);

        if let Some(ref right) = node.right {
            Solution::inorder_traversal_inner(values, right);
        }
    }
}
