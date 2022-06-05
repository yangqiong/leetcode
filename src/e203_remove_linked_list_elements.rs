// https://leetcode.cn/problems/remove-linked-list-elements/
// 给你一个链表的头节点 head 和一个整数 val ，请你删除链表中所有满足 Node.val == val 的节点，并返回 新的头节点 。

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
//
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}
impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            if node.val == val {
                Solution::remove_elements(node.next, val)
            } else {
                node.next = Solution::remove_elements(node.next, val);
                Some(node)
            }
        } else {
            None
        }
    }
}
