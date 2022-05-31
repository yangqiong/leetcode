// https://leetcode.cn/problems/balanced-binary-tree/
// 给定一个二叉树，判断它是否是高度平衡的二叉树。
// 本题中，一棵高度平衡二叉树定义为：
// 一个二叉树每个节点 的左右两个子树的高度差的绝对值不超过 1

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
use std::cmp::max;
use std::rc::Rc;

struct Solution {}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_balanced_node(root.as_ref())
    }

    pub fn is_balanced_node(node: Option<&Rc<RefCell<TreeNode>>>) -> bool {
        match node {
            None => true,
            Some(node) => {
                let left_height = Self::height(node.borrow().left.as_ref());
                let right_height = Self::height(node.borrow().right.as_ref());
                (left_height - right_height).abs() <= 1
                    && Self::is_balanced_node(node.borrow().left.as_ref())
                    && Self::is_balanced_node(node.borrow().right.as_ref())
            }
        }
    }

    fn height(node: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
        match node {
            None => 0,
            Some(node) => {
                max(
                    Self::height(node.borrow().left.as_ref()),
                    Self::height(node.borrow().right.as_ref()),
                ) + 1
            }
        }
    }
}
