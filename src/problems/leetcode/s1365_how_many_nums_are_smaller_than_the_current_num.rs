//! # 1365. How many numbers are smaller than the current number.
//! 来源: LeetCode | 链接: https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number
//! 难度: Easy ｜ 标签: Array
//! 归属: Quest · Array II · Q2 | Date: 2026-11-22

pub struct Solution;

impl Solution {
    pub fn smaller_numbers_than_current(nums: Vec<i32>) -> Vec<i32> {
        // A: 个人实现
        //let n = nums.len();
        //let mut ans = Vec::with_capacity(n);
        //let mut nums_count = vec![0; 101];
        //for &x in &nums {
        //    let idx = x as usize;
        //    nums_count[idx] += 1;
        //}
        //for i in 1..=100 {
        //    nums_count[i] += nums_count[i - 1];
        //}
        //for &x in nums.iter() {
        //    let num = if x == 0 {
        //        0
        //    } else {
        //        let idx = x as usize;
        //        nums_count[idx - 1]
        //    };
        //    ans.push(num);
        //}
        //ans

        // B: 更rust 的版本
        //let mut cnt = [0i32; 101];
        //for &x in &nums {
        //    cnt[x as usize] += 1;
        //}

        //let mut acc = 0;
        //for slot in &mut cnt {
        //    let c = std::mem::replace(slot, acc);
        //    acc += c;
        //}

        //nums.iter().map(|&x| cnt[x as usize]).collect()

        // C: 排序 + 二分查找 空间 O(N)  时间 O(n log n)
        //let mut sorted = nums.clone();
        //sorted.sort_unstable();

        //nums.iter()
        //    .map(|&x| sorted.partition_point(|&y| y < x) as i32)
        //    .collect()

        // D: 排序 + Hash 表 O(N)  时间 O(n log n)
        //let mut sorted = nums.clone();
        //sorted.sort_unstable();
        //let rank: std::collections::HashMap<i32, usize> = sorted
        //    .iter()
        //    .enumerate()
        //    .filter(|&(i, &x)| i == 0 || sorted[i - 1] != x)
        //    .map(|(i, &x)| (x, i))
        //    .collect();

        //nums.iter().map(|&x| rank[&x] as i32).collect()

        // E: 排序 + 排名映射 O(N)  时间 O(n log n)
        let mut indexed: Vec<(i32, usize)> = nums.iter().copied().zip(0..).collect(); // zip 会自动把数组转成迭代器用于合并
        indexed.sort_unstable_by_key(|&(v, _)| v);

        let mut ans = vec![0; nums.len()];
        for (rank, &(val, original_idx)) in indexed.iter().enumerate() {
            if rank > 0 && indexed[rank - 1].0 == val {
                ans[original_idx] = ans[indexed[rank - 1].1];
            } else {
                ans[original_idx] = rank as i32;
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
            Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
            vec![4, 0, 1, 1, 3]
        )
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![6, 5, 4, 8]),
            vec![2, 1, 0, 3]
        )
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::smaller_numbers_than_current(vec![5, 0, 10, 0, 10, 6]),
            vec![2, 0, 4, 0, 4, 3]
        )
    }
}
