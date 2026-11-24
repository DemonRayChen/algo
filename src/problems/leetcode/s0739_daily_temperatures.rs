//! # 0739. Daily Temperatures
//! 来源: LeetCode | 链接: https://leetcode.com/problems/daily-temperatures
//! 难度: Medium ｜ 标签: Array, Monotonic Stack
//! 归属: Quest · Monotonic Stack - Q2 | Date: 2026-11-24

pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut ans: Vec<i32> = vec![0; n];
        let mut stack = Vec::new();

        for i in 0..n {
            while let Some(&top) = stack.last() {
                if temperatures[top] < temperatures[i] {
                    let popped = stack.pop().unwrap();
                    ans[popped] = (i - top) as i32;
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0]
        );
    }
}
