//! # Problem 6: Sum Square Difference
//!
//! **Difficulty:** ⭐
//! **Link:** <https://projecteuler.net/problem=6>
//!
//! ## Problem Statement
//!
//! The sum of the squares of the first ten natural numbers is,
//! 1² + 2² + ... + 10² = 385
//!
//! The square of the sum of the first ten natural numbers is,
//! (1 + 2 + ... + 10)² = 55² = 3025
//!
//! Hence the difference between the sum of the squares of the first ten
//! natural numbers and the square of the sum is 3025 − 385 = 2640.
//!
//! Find the difference between the sum of the squares of the first one
//! hundred natural numbers and the square of the sum.
//!
//! ## Mathematical Solution
//!
//! 利用伯努利数公式计算幂次和：
//!
//! $$\sum_{k=1}^{n} k^p = \frac{1}{p+1} \sum_{j=0}^{p} \binom{p+1}{j} B_j \cdot n^{p+1-j}$$
//!
//! - `sum_of_squares` = `power_sum(n, 2)`（p=2 的伯努利求和）
//! - `square_of_sum` = `arithmetic_series_sum(1, 1, n)²`（等差数列求和后平方）
//! - 答案 = `square_of_sum - sum_of_squares`
//!
//! ## Complexity
//!
//! - Time: $O(p^2)$，p=2 时实际 O(1)
//! - Space: $O(p)$

use pe::math::{arithmetic_series_sum, power_sum};

fn solve() -> i64 {
    let n = 100u64;
    let square_of_sum = arithmetic_series_sum(1, 1, n).pow(2);
    let sum_of_squares = power_sum(n, 2);
    square_of_sum - sum_of_squares
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // n=10: square_of_sum=3025, sum_of_squares=385, diff=2640
        use pe::math::{arithmetic_series_sum, power_sum};
        let n = 10u64;
        let diff = arithmetic_series_sum(1, 1, n).pow(2) - power_sum(n, 2);
        assert_eq!(diff, 2640);
    }

    #[test]
    fn test_answer() {
        assert_eq!(solve(), 25164150);
    }
}
