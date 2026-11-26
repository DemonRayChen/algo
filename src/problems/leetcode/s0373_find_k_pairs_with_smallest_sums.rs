//! # 0373. Find K Pairs with Smallest Sums
//! 来源: LeetCode | 链接: https://leetcode.com/problems/find-k-pairs-with-smallest-sums/description/
//! 难度: Easy ｜ 标签: Array Heap(Priority Queue)
//! 归属: Quest · Heap - Q2 | Date: 2026-11-26

use std::{cmp::Reverse, collections::BinaryHeap, vec};

pub struct Solution;

impl Solution {
    pub fn k_smallest(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        // 暴力会爆内存,也很慢
        // 小根堆, 空间 O(k)  时间 O(k log k)
        let k = k as usize;
        if nums1.is_empty() || nums2.is_empty() {
            return vec![];
        }

        let mut heap = BinaryHeap::new();
        let mut result = Vec::new();

        for i in 0..nums1.len().min(k) {
            heap.push(Reverse((nums1[i] + nums2[0], i, 0usize)));
        }
        while result.len() < k {
            let Some(Reverse((_, i, j))) = heap.pop() else {
                break;
            };

            result.push(vec![nums1[i], nums2[j]]);

            if j + 1 < nums2.len() {
                heap.push(Reverse((nums1[i] + nums2[j + 1], i, j + 1)));
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::k_smallest(vec![1, 7, 11], vec![2, 4, 6], 3),
            vec![vec![1, 2], vec![1, 4], vec![1, 6]]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::k_smallest(vec![1, 1, 2], vec![1, 2, 3], 2),
            vec![vec![1, 1], vec![1, 1]]
        );
    }
}
