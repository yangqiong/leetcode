// https://leetcode.cn/problems/merge-two-binary-trees/
// 给你两棵二叉树： root1 和 root2 。
// 想象一下，当你将其中一棵覆盖到另一棵之上时，两棵树上的一些节点将会重叠（而另一些不会）。你需要将这两棵树合并成一棵新二叉树。合并的规则是：如果两个节点重叠，那么将这两个节点的值相加作为合并后节点的新值；否则，不为 null 的节点将直接作为新二叉树的节点。
// 返回合并后的二叉树。

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
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (Some(root1), Some(root2)) => Some(Rc::new(RefCell::new(TreeNode {
                val: root1.borrow().val + root2.borrow().val,
                left: Self::merge_trees(
                    root1.as_ref().borrow().left.clone(),
                    root2.as_ref().borrow().left.clone(),
                ),
                right: Self::merge_trees(
                    root1.as_ref().borrow().right.clone(),
                    root2.as_ref().borrow().right.clone(),
                ),
            }))),
            (Some(root1), None) => Some(Rc::new(RefCell::new(TreeNode {
                val: root1.borrow().val,
                left: root1.as_ref().borrow().left.clone(),
                right: root1.as_ref().borrow().right.clone(),
            }))),
            (None, Some(root2)) => Some(Rc::new(RefCell::new(TreeNode {
                val: root2.borrow().val,
                left: root2.as_ref().borrow().left.clone(),
                right: root2.as_ref().borrow().right.clone(),
            }))),
            (None, None) => None,
        }
    }
}
