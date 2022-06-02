// https://leetcode.cn/problems/binary-tree-paths/
// 给你一个二叉树的根节点 root ，按 任意顺序 ，返回所有从根节点到叶子节点的路径。

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
use std::collections::VecDeque;
use std::rc::Rc;
struct Solution {}
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();
        let mut line: Vec<i32> = Vec::new();
        Self::binary_tree_paths_node(root.as_ref(), &mut results, line.clone());
        results
    }

    pub fn binary_tree_paths_node(
        node: Option<&Rc<RefCell<TreeNode>>>,
        results: &mut Vec<String>,
        line: Vec<i32>,
    ) {
        let mut line = line.clone();
        match node {
            None => (),
            Some(node) => {
                line.push(node.borrow().val);
                if node.borrow().left.is_some() {
                    Self::binary_tree_paths_node(
                        node.borrow().left.as_ref(),
                        results,
                        line.clone(),
                    );
                }
                if node.borrow().right.is_some() {
                    Self::binary_tree_paths_node(
                        node.borrow().right.as_ref(),
                        results,
                        line.clone(),
                    );
                }
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    let result: Vec<String> = line.iter().map(|&n| n.to_string()).collect();
                    results.push(result.join("->"));
                }
            }
        }
    }
}
