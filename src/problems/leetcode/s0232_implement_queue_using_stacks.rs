//! # 0232. Implement Queue using Stacks
//! 来源: LeetCode | 链接: https://leetcode.com/problems/implement-queue-using-stacks/description/
//! 难度: Easy ｜ 标签: Stack Queue Simulation
//! 归属: Quest · Queue - Q3 | Date: 2026-11-25

struct MyQueue {
    stack_in: Vec<i32>,
    stack_out: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {
    fn new() -> Self {
        Self {
            stack_in: Vec::new(),
            stack_out: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.stack_in.push(x);
    }

    fn pop(&mut self) -> i32 {
        self.peek();
        self.stack_out.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if self.stack_out.is_empty() {
            while let Some(x) = self.stack_in.pop() {
                self.stack_out.push(x);
            }
        }

        *self.stack_out.last().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack_in.is_empty() && self.stack_out.is_empty()
    }
}
