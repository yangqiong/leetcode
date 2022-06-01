// https://leetcode.cn/problems/invert-binary-tree/
// 给你一棵二叉树的根节点 root ，翻转这棵二叉树，并返回其根节点。

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
use std::mem::swap;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::invert_tree_node(root.as_ref());
        root
    }

    pub fn invert_tree_node(node: Option<&Rc<RefCell<TreeNode>>>) {
        match node {
            None => (),
            Some(node) => {
                let left = node.borrow_mut().left.take();
                let right = node.borrow_mut().right.take();
                node.borrow_mut().left = right;
                node.borrow_mut().right = left;
                Self::invert_tree_node(node.borrow().left.as_ref());
                Self::invert_tree_node(node.borrow().right.as_ref());
            }
        }
    }
}
