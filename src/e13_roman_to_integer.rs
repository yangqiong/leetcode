// https://leetcode.cn/problems/roman-to-integer/
// 罗马数字转整数

struct Solution {}
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result: i32 = 0;
        let chars: Vec<char> = s.chars().collect();
        let length = chars.len();
        let mut index = 0;
        loop {
            let mut skip_next = false;
            if index == length {
                break;
            }
            if let Some(&c) = chars.get(index) {
                match c {
                    'I' => {
                        if let Some(&c) = chars.get(index + 1) {
                            match c {
                                'V' => {
                                    result += 4;
                                    skip_next = true;
                                }
                                'X' => {
                                    result += 9;
                                    skip_next = true;
                                }
                                _ => result += 1,
                            }
                        } else {
                            result += 1
                        }
                    }
                    'V' => result += 5,
                    'X' => {
                        if let Some(&c) = chars.get(index + 1) {
                            match c {
                                'L' => {
                                    result += 40;
                                    skip_next = true;
                                }
                                'C' => {
                                    result += 90;
                                    skip_next = true;
                                }
                                _ => result += 10,
                            }
                        } else {
                            result += 10
                        }
                    }
                    'L' => result += 50,
                    'C' => {
                        if let Some(&c) = chars.get(index + 1) {
                            match c {
                                'D' => {
                                    result += 400;
                                    skip_next = true;
                                }
                                'M' => {
                                    result += 900;
                                    skip_next = true;
                                }
                                _ => result += 100,
                            }
                        } else {
                            result += 100
                        }
                    }
                    'D' => result += 500,
                    'M' => result += 1000,
                    _ => (),
                };
            }
            if skip_next == true {
                index += 2;
            } else {
                index += 1;
            }
        }
        result
    }
}
