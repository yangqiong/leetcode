use std::result;

struct Solution {}

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut result = vec![1; (row_index + 1) as usize];
        let row_index = row_index as usize;
        for i in 1..=row_index {
            let mid = (i + 2) / 2;
            for j in (1..mid).rev() {
                result[j] = result[j] + result[j - 1];
                result[i - j] = result[j];
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_e119() {
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
    }

    #[test]
    fn it_works_e119_test2() {
        assert_eq!(Solution::get_row(4), vec![1, 4, 6, 4, 1]);
    }
}
