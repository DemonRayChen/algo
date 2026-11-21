//! # 0645. Set Mismatch
//! 来源: LeetCode | 链接:  https://leetcode.com/problems/shuffle-the-array
//! 难度: Easy ｜ 标签: Array
//! 归属: Quest · Array II · Q1 | Date: 2026-11-20

pub struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len() + 1;
        let mut flags = vec![false; n];
        let mut duplicate = 0;

        for &x in &nums {
            if flags[x as usize] {
                duplicate = x;
                break;
            }
            flags[x as usize] = true;
        }

        let mut missing = 0;
        for i in 1..n {
            if !flags[i] {
                missing = i as i32;
            }
        }

        vec![duplicate, missing]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::find_error_nums(vec![1, 1]), vec![1, 2]);
    }
}
