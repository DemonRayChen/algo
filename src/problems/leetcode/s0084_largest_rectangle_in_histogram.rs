//! # 0084. Largest Rectangle in Histogram
//! 来源: LeetCode | 链接: https://leetcode.com/problems/largest-rectangle-in-histogram
//! 难度: Hard ｜ 标签: Array, Monotonic Stack, Range Minimum/Maximum Query
//! 归属: Quest · Monotonic Stack - Q3 | Date: 2026-11-24

pub struct Solution;

impl Solution {
    pub fn largest_rectangle_in_histogram(heights: Vec<i32>) -> i32 {
        // 使用递增栈
        let mut ans = 0;
        let mut stack: Vec<usize> = Vec::new();
        let n = heights.len();
        for i in 0..=n {
            // 下标到达 i 时计算的是 i - 1 之前的矩形
            let cur_h = if i < n { heights[i] } else { 0 };
            while let Some(&top) = stack.last() {
                if heights[top] > cur_h {
                    let h = heights[top]; // 单调递增, 所以 h 是这个栈里面 到 当前下标 i 这几个范围内的最小高度,所以直接用作高度参与计算
                    stack.pop();
                    let left = stack.last().map_or(0, |&l| l + 1);
                    let width = (i - left) as i32;
                    ans = ans.max(h * width);
                } else {
                    break;
                }
            }
            if i < n {
                stack.push(i);
            }
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
            Solution::largest_rectangle_in_histogram(vec![2, 1, 5, 6, 2, 3]),
            10
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::largest_rectangle_in_histogram(vec![2, 4]), 4);
    }
}
