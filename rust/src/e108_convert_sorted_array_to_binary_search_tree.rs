// https://leetcode.cn/problems/convert-sorted-array-to-binary-search-tree/
// 给你一个整数数组 nums ，其中元素已经按 升序 排列，请你将其转换为一棵 高度平衡 二叉搜索树。
// 高度平衡 二叉树是一棵满足「每个节点的左右两个子树的高度差的绝对值不超过 1 」的二叉树。

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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 0 {
            return None;
        }
        Self::createBST(&nums, 0, nums.len() - 1)
    }

    fn createBST(nums: &Vec<i32>, start: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>> {
        let mid = (start + end) / 2;
        let mut node = TreeNode {
            val: nums[mid],
            left: None,
            right: None,
        };
        if mid > start {
            node.left = Self::createBST(nums, start, mid - 1)
        }
        if mid < end {
            node.right = Self::createBST(nums, mid + 1, end)
        }
        return Some(Rc::new(RefCell::new(node)));
    }
}
