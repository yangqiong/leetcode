struct Solution {}

use std::cmp::{max, min};

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let count = height.len();
        let mut i = 0;
        let mut j = count - 1;
        let mut result = 0;
        while j > i {
            result = max(result, (j - i) as i32 * min(height[i], height[j]));
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_e11() {
        assert_eq!(Solution::max_area(vec![2, 3, 4, 5, 18, 17, 6]), 17);
    }
}
