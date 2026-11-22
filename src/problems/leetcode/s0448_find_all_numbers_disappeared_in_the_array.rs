//! # 0448. Find All Numbers Disappeared in an Array
//! 来源: LeetCode | 链接: https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array
//! 难度: Easy ｜ 标签: Array
//! 归属: Quest · Array II · Q3 | Date: 2026-11-22

pub struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        // A: 排序加判断 时间 O(n log n) 空间 O(n)
        //let n = nums.len();
        //let mut nums = nums;
        //nums.sort_unstable();
        //let mut ans = Vec::with_capacity(n);
        //for i in 1..=n {
        //    if !nums.binary_search(&(i as i32)).is_ok() {
        //        ans.push(i as i32);
        //    }
        //}
        //ans

        // B: 标志位两边循环 时间 O(n) 空间 O(n)
        //let n = nums.len();
        //let mut existed = vec![false; n + 1];
        //let mut ans = Vec::with_capacity(n);
        //for &x in &nums {
        //    existed[x as usize] = true;
        //}
        //existed[0] = true;
        //for (idx, &x) in existed.iter().enumerate() {
        //    if !x {
        //        ans.push(idx as i32);
        //    }
        //}
        //ans

        // C: 原地标记:
        let mut nums = nums;
        for i in 0..nums.len() {
            let idx = (nums[i].abs() - 1) as usize;
            nums[idx] = -nums[idx].abs();
        }

        (0..nums.len())
            .filter(|&i| nums[i] > 0)
            .map(|i| (i + 1) as i32)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        )
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::find_disappeared_numbers(vec![1, 1]), vec![2])
    }
}
