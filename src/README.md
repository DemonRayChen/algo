# algo

个人算法与 Rust 双修练习库：一边补数据结构与算法（LeetCode Learn 卡片 / LeetCode 75 / Top Interview 150 / Quest），一边练 Rust（所有权、标准库、模块化工程）。

## 目录结构

```
algo/
├── Cargo.toml
├── README.md
├── notes/                  # 概念笔记
├── tests/                  # 集成测试
└── src/
    ├── lib.rs              # 库入口：声明各模块
    ├── main.rs             # 二进制入口：题目试跑 / 模板生成
    ├── ds.rs  + ds/        # 数据结构实现
    ├── patterns.rs + patterns/  # 算法模式模板
    ├── problems.rs + problems/  # 题目解答
    └── util.rs + util/     # 共享类型与测试工具
```

## 各目录职责

| 目录 | 职责 | 对应学习内容 |
| --- | --- | --- |
| `src/ds/` | **数据结构的亲手实现**：链表、二叉树、堆、Trie、并查集等，追求地道 Rust 写法，用于吃透所有权与 API 设计 | LeetCode Learn 卡片（Linked List / Hash Table / Binary Tree …） |
| `src/patterns/` | **可复用的算法模式模板**：二分边界、滑动窗口、回溯、DP 骨架等，只放"解题骨架 + 单元测试"，不放具体题解 | 算法技巧专项 |
| `src/problems/` | **LeetCode 题目解答**：每题一个文件，保持 `Solution` 签名以便复制到网站提交 | LeetCode 75 / Top Interview 150 / Quest / 每日一题 |
| `src/util/` | **共享工具**：与 LeetCode 签名兼容的 `ListNode`、`TreeNode` 定义，`vec → 链表` 等测试辅助函数 | — |
| `notes/` | **概念笔记**（markdown）：按主题记录思路、易错点、"这题为什么没想到"，文件与 `ds/` 章节对应 | 复习索引 |
| `tests/` | **集成测试**：跨模块的对拍与回归测试，检验 `pub` 接口是否完整 | — |

依赖方向单向：`problems/` 可引用 `patterns/`、`ds/`、`util/`；`patterns/` 不依赖 `problems/`。

## 命名与代码约定

- **题目文件**：`s{四位编号}_{题名}.rs`，如 `s0001_two_sum.rs`，与 LeetCode 编号对齐，便于检索。
- **题目文件头部注释**（元数据，方便日后按标签复习）：

  ```rust
  //! # 1. Two Sum
  //! 链接: https://leetcode.com/problems/two-sum/
  //! 难度: Easy | 标签: Array, Hash Table
  //! 归属: LeetCode 75 · 数组专题
  //! 模式: sliding_window / hash_lookup
  //! 日期: 2026-11-20
  ```

- **签名保持 LeetCode 原样**（`pub struct Solution;` + `impl Solution`），测试写在同文件 `#[cfg(test)] mod tests`。
- **模块风格**：统一使用 `ds.rs` + `ds/` 的现代风格，不使用 `mod.rs`。

## 常用命令

```bash
cargo test              # 全量测试
cargo test s0001        # 只跑某题的测试
cargo run               # 运行 main.rs（题目试跑）
cargo fmt && cargo clippy -- -D warnings   # 提交前检查
```
