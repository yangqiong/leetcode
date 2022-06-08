struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // f(n) = fn(n-1) + fn(n-2)
        match n {
            1 => 1,
            2 => 2,
            _ => {
                let mut f_i = 0;
                let mut f_1 = 1;
                let mut f_2 = 2;
                for _ in 3..=n {
                    f_i = f_1 + f_2;
                    f_1 = f_2;
                    f_2 = f_i;
                }
                f_i
            }
        }
    }
}
