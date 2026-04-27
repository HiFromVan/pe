//! # Problem 5: Smallest Multiple
//!
//! **Difficulty:** ⭐
//! **Link:** <https://projecteuler.net/problem=5>
//!
//! ## Problem Statement
//!
//! 2520 is the smallest number that can be divided by each of the numbers
//! from 1 to 10 without any remainder.
//!
//! What is the smallest positive number that is evenly divisible by all
//! of the numbers from 1 to 20?
//!
//! ## Mathematical Solution
//! 那么这么思路是什么
//! 穷举法：2520一直加2520然后/(除数列表:11-20)
//! 1-20每个素数的最大次幂(小于 20)相乘
//!
//! ## Complexity
//!
//! - Time: $O(?)$
//! - Space: $O(?)$

use pe::math::{is_prime, lcm};

fn solve() -> u64 {
    (2..=20u64).fold(1,  lcm)
}

fn solve1() -> u64 {
    let limit = 20u64;
    let mut result: u64 = 1;
    for n in 2..=limit {
        if is_prime(n) {
            let mut p_pow = n;
            while p_pow * n <= limit {
                p_pow *= n;
            }
            result *= p_pow;
        }
    }
    result
}

fn main() {
    let answer = solve1();
    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(solve(), 232792560);
        assert_eq!(solve1(), 232792560);
    }
}
