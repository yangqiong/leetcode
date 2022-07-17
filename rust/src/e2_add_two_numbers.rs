// https://leetcode.cn/problems/add-two-numbers/
// 给你两个 非空 的链表，表示两个非负的整数。它们每位数字都是按照 逆序 的方式存储的，并且每个节点只能存储 一位 数字。
// 请你将两个数相加，并以相同形式返回一个表示和的链表。
// 你可以假设除了数字 0 之外，这两个数都不会以 0 开头。

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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Self::dfs(l1, l2, 0)
    }

    fn dfs(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        mut carried: i32,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() && carried == 0 {
            return None;
        }
        Some(Box::new(ListNode {
            next: Self::dfs(
                l1.and_then(|x| {
                    carried += x.val;
                    x.next
                }),
                l2.and_then(|x| {
                    carried += x.val;
                    x.next
                }),
                carried / 10,
            ),
            val: carried % 10,
        }))
    }
}
