//! # 1354. Construct Target Array With Multiple Sums
//! 来源: LeetCode | 链接: https://leetcode.com/problems/construct-target-array-with-multiple-sums/description/
//! 难度: Hard ｜ 标签: Array Heap(Priority Queue)
//! 归属: Quest · Heap - Q3 | Date: 2026-11-26

pub struct Solution;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        use std::collections::BinaryHeap;

        let n = target.len();
        if n == 1 {
            return target[0] == 1;
        }

        let mut sum: i64 = target.iter().map(|&x| x as i64).sum();
        let mut heap: BinaryHeap<i64> = target.iter().map(|&x| x as i64).collect();

        loop {
            let max = *heap.peek().unwrap();

            //
            if max == 1 {
                return true;
            }

            let rest = sum - max;

            // n == 2 时, 可以在 rest == 1 时返回true
            if rest == 1 {
                return true;
            }

            if rest == 0 || max <= rest || max % rest == 0 {
                return false;
            }

            // 这里是把 当前最大值, 降低到使用 其他元素的总和作为加数之前的状态
            let prev = max % rest;
            sum = rest + prev; // 新的最大值
            heap.pop();
            heap.push(prev);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::is_possible(vec![9, 3, 5]), true);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::is_possible(vec![1, 1, 1, 2]), false);
    }
}
