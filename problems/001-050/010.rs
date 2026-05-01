//! # Problem 10: Summation of Primes
//!
//! **Difficulty:** ⭐
//! **Link:** <https://projecteuler.net/problem=10>
//!
//! ## Problem Statement
//!
//! The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
//!
//! Find the sum of all the primes below two million.
//!
//! ## Mathematical Solution
//!
//! 埃式筛法筛出 2,000,000 以内所有素数，直接求和。
//! 与第 7 题相同的筛法，时间复杂度 $O(n \log \log n)$，空间 $O(n)$。

use pe::math::sieve_of_eratosthenes;

fn solve() -> u64 {
    let limit = 1_999_999;
    sieve_of_eratosthenes(limit)
        .iter().enumerate()
        .filter(|&(_, &p)| p)
        .map(|(i, _)| i as u64)
        .sum()
}

fn main() {
    println!("{}", solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        use pe::math::sieve_of_eratosthenes;
        let sum: u64 = sieve_of_eratosthenes(9)
            .iter().enumerate()
            .filter(|&(_, &p)| p)
            .map(|(i, _)| i as u64)
            .sum();
        assert_eq!(sum, 17);
    }

    #[test]
    fn test_answer() {
        assert_eq!(solve(), 142913828922);
    }
}
