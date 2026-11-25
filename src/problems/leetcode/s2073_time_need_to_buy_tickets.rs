//! # 2703. Time Needed to Buy Tickets
//! 来源: LeetCode | 链接: https://leetcode.com/problems/time-needed-to-buy-tickets/description/
//! 难度: Easy ｜ 标签: Array Queue Simulation
//! 归属: Quest · Queue - Q2 | Date: 2026-11-25

pub struct Solution;

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        // A:
        // let tigcccket_count = tickets[k as usize];
        // let mut ans = ticket_count;
        // for i in 0..k as usize {
        //     if tickets[i] < ticket_count {
        //         ans += tickets[i];
        //     } else {
        //         ans += ticket_count;
        //     }
        // }
        // for i in (k as usize + 1)..tickets.len() {
        //     if tickets[i] < ticket_count - 1 {
        //         ans += tickets[i];
        //     } else {
        //         ans += ticket_count - 1;
        //     }
        // }
        // ans

        // B
        let (k, t) = (k as usize, tickets[k as usize]);

        tickets
            .iter()
            .enumerate()
            .map(|(i, &x)| x.min(if i <= k { t } else { t - 1 }))
            .sum()

        // C 模拟队列,这里不想写代码了
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::time_required_to_buy(vec![2, 3, 2], 2), 6);
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::time_required_to_buy(vec![5, 1, 1, 1], 0), 8);
    }
}
