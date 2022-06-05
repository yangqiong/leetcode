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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut nums: Vec<i32> = Vec::new();
        let mut l1 = list1.as_ref();
        let mut l2 = list2.as_ref();
        loop {
            match (l1, l2) {
                (Some(n1), Some(n2)) => {
                    if n1.val < n2.val {
                        nums.push(n1.val);
                        l1 = n1.next.as_ref();
                    } else {
                        nums.push(n2.val);
                        l2 = n2.next.as_ref();
                    }
                }
                (Some(n1), None) => {
                    nums.push(n1.val);
                    l1 = n1.next.as_ref();
                }
                (None, Some(n2)) => {
                    nums.push(n2.val);
                    l2 = n2.next.as_ref();
                }
                _ => break,
            }
        }

        create_list(&nums, 0)
    }
}

fn create_list(nums: &Vec<i32>, index: usize) -> Option<Box<ListNode>> {
    if let Some(num) = nums.get(index) {
        Some(Box::new(ListNode {
            val: *num,
            next: create_list(nums, index + 1),
        }))
    } else {
        None
    }
}
