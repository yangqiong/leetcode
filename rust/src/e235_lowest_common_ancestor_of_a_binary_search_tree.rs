// https://leetcode.cn/problems/lowest-common-ancestor-of-a-binary-search-tree/
// 给定一个二叉搜索树, 找到该树中两个指定节点的最近公共祖先。

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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (p, q) {
            (Some(p), Some(q)) => {
                Self::lowest_common_ancestor_node(root.as_ref(), p.borrow().val, q.borrow().val)
            }
            _ => None,
        }
    }

    fn lowest_common_ancestor_node(
        node: Option<&Rc<RefCell<TreeNode>>>,
        p: i32,
        q: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match node {
            Some(node) => {
                if node.borrow().val > p && node.borrow().val > q {
                    return Self::lowest_common_ancestor_node(node.borrow().left.as_ref(), p, q);
                }
                if node.borrow().val < p && node.borrow().val < q {
                    return Self::lowest_common_ancestor_node(node.borrow().right.as_ref(), p, q);
                }
                Some(Rc::new(RefCell::new(TreeNode::new(node.borrow().val))))
            }
            None => None,
        }
    }
}
