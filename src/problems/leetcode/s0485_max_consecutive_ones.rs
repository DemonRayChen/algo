//! # 1470. Max Consecutive Ones
//! 来源: LeetCode | 链接: https://leetcode.com/problems/max-consecutive-ones
//! 难度: Easy ｜ 标签: Array
//! 归属: Quest · Array I · Q3 | Date: 2026-11-21

pub struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        // A: 单趟扫描
        //let n = nums.len();
        //let mut ans = 0;
        //let mut tmp = 0;
        //for i in 0..n {
        //    if nums[i] == 1 {
        //        tmp += 1;
        //        ans = std::cmp::max(tmp, ans);
        //    } else {
        //        tmp = 0;
        //    }
        //}
        //ans

        // B: split 切片求最长
        //nums.split(|&x| x == 0)
        //    .map(<[i32]>::len) // 这里的 map 处理的元素的类型是切片类型
        //    .max()
        //    .unwrap_or(0) as i32

        // C: fold 状态机
        //nums.iter()
        //    .fold((0, 0), |(run, best), &x| {
        //        if x == 1 {
        //            (run + 1, best.max(run + 1))
        //        } else {
        //            (0, best)
        //        }
        //    })
        //    .1

        // D: scan
        //nums.iter()
        //    .scan(0, |run, &x| {
        //        *run = if x == 1 { *run + 1 } else { 0 };
        //        Some(*run)
        //    })
        //    .max()
        //    .unwrap_or(0)

        // E: chunk_by + filter
        // nums.chunk_by(|&a, &b| a == b)
        //    .filter(|chunk| chunk[0] == 1)
        //    .map(<[i32]>::len)
        //    .max()
        //    .unwrap_or(0) as i32

        // F: 更 rust 的编写方法
        let mut run = 0;
        let mut best = 0;
        for &x in &nums {
            run = if x == 1 { run + 1 } else { 0 };
            best = best.max(run);
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]),
            3
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]),
            2
        );
    }
}
