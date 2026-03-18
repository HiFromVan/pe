//! # Problem 1: Multiples of 3 or 5
//!
//! **Difficulty:** ⭐
//! **Link:** <https://projecteuler.net/problem=1>
//!
//! ## Problem Statement
//!
//! If we list all the natural numbers below 10 that are multiples of 3 or 5,
//! we get 3, 5, 6 and 9. The sum of these multiples is 23.
//!
//! Find the sum of all the multiples of 3 or 5 below 1000.
//!
//! ## Mathematical Solution
//!
//! The sum of multiples of $k$ below $m$ is given by the arithmetic series formula:
//!
//! $$S_k = k \cdot \frac{n(n+1)}{2}, \quad \text{where } n = \lfloor m/k \rfloor$$
//!
//! By the inclusion-exclusion principle:
//!
//! $$\text{Answer} = S_3 + S_5 - S_{15}$$
//!
//! Expanding this:
//!
//! $$ S = \underbrace{\frac{3 \cdot \lfloor \frac{999}{3} \rfloor (1 + \lfloor \frac{999}{3} \rfloor)}{2}}_{3\text{的倍数之和}} + \underbrace{\frac{5 \cdot \lfloor \frac{999}{5} \rfloor (1 + \lfloor \frac{999}{5} \rfloor)}{2}}_{5\text{的倍数之和}} - \underbrace{\frac{15 \cdot \lfloor \frac{999}{15} \rfloor (1 + \lfloor \frac{999}{15} \rfloor)}{2}}_{15\text{的倍数之和}} $$
//!
//! ## Complexity
//!
//! - Time: $O(1)$
//! - Space: $O(1)$
//!
//! ## Example
//!
//! ```rust
//! # fn sum_multiples(k: u64, limit: u64) -> u64 {
//! #     let n = limit / k;
//! #     k * n * (n + 1) / 2
//! # }
//! # fn solve(limit: u64) -> u64 {
//! #     sum_multiples(3, limit) + sum_multiples(5, limit) - sum_multiples(15, limit)
//! # }
//! assert_eq!(solve(9), 23);
//! ```

/// Calculate the sum of multiples of k below or equal to limit
pub fn sum_multiples(k: u64, limit: u64) -> u64 {
    let n = limit / k;
    k * n * (n + 1) / 2
}

/// Solve Problem 1
pub fn solve(limit: u64) -> u64 {
    sum_multiples(3, limit) + sum_multiples(5, limit) - sum_multiples(15, limit)
}

fn main() {
    let answer = solve(999);
    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(9), 23);
    }

    #[test]
    fn test_answer() {
        assert_eq!(solve(999), 233168);
    }
}
