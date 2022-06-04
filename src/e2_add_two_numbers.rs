use core::num;

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
        let mut nums: Vec<i32> = Vec::new();
        let mut plus = 0;
        let mut cur1 = l1.as_ref();
        let mut cur2 = l2.as_ref();

        while cur1.is_some() && cur2.is_some() {
            let val1 = cur1.unwrap().val;
            let val2 = cur2.unwrap().val;
            let sum = val1 + val2 + plus;
            if sum >= 10 {
                nums.push(sum - 10);
                plus = 1;
            } else {
                nums.push(sum);
                plus = 0;
            }
            cur1 = cur1.unwrap().next.as_ref();
            cur2 = cur2.unwrap().next.as_ref();
        }

        while cur1.is_some() {
            let val1 = cur1.unwrap().val;
            let sum = val1 + plus;
            if sum >= 10 {
                nums.push(sum - 10);
                plus = 1;
            } else {
                nums.push(sum);
                plus = 0;
            }
            cur1 = cur1.unwrap().next.as_ref();
        }

        while cur2.is_some() {
            let val2 = cur2.unwrap().val;
            let sum = val2 + plus;
            if sum >= 10 {
                nums.push(sum - 10);
                plus = 1;
            } else {
                nums.push(sum);
                plus = 0;
            }
            cur2 = cur2.unwrap().next.as_ref();
        }

        if plus == 1 {
            nums.push(plus);
        }

        nums.reverse();

        let mut head = None;
        for i in 0..nums.len() {
            let mut node = ListNode::new(nums[i]);
            node.next = head;
            head = Some(Box::new(node))
        }

        head
    }
}
