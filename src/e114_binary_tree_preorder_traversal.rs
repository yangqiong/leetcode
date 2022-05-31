// https://leetcode.cn/problems/binary-tree-preorder-traversal/
// 给你二叉树的根节点 root ，返回它节点值的 前序 遍历。

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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut results: Vec<i32> = Vec::new();

        Self::preorder_traversal_node(root.as_ref(), &mut results);

        results
    }

    fn preorder_traversal_node(node: Option<&Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        match node {
            None => (),
            Some(node) => {
                result.push(node.borrow().val);
                Self::preorder_traversal_node(node.borrow().left.as_ref(), result);
                Self::preorder_traversal_node(node.borrow().right.as_ref(), result);
            }
        }
    }
}
