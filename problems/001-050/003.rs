//! # Problem 3: Largest Prime Factor
//!
//! **Difficulty:** ⭐
//! **Link:** <https://projecteuler.net/problem=3>
//!
//! ## Problem Statement
//!
//! The prime factors of 13195 are 5, 7, 13 and 29.
//!
//! What is the largest prime factor of the number 600851475143?
//!
//! ## Mathematical Solution
//!
//! 从 2 开始试除，每次找到因数就一直除干净，再继续下一个数。
//! 由于合数的所有质因数都已被提前除掉，能整除 $n$ 的必然是质数。
//!
//! 只需试到 $\sqrt{n}$：若 $n$ 还有因数，其中必有一个 $\leq \sqrt{n}$。
//! 循环结束后若 $n > 1$，则剩余的 $n$ 本身是最大质因数。
//!
//! ## Complexity
//!
//! - Time: $O(\sqrt{n})$
//! - Space: $O(1)$

fn solve(mut n: u64) -> u64 {
    let mut last = 1;
    let mut factor = 2;
    while factor * factor <= n {
        while n % factor == 0 {
            last = factor;
            n /= factor;
        }
        factor += 1;
    }
    if n > 1 { n } else { last }
}

fn main() {
    let answer = solve(600_851_475_143);
    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solve(13195), 29);
    }

    #[test]
    fn test_answer() {
        assert_eq!(solve(600_851_475_143), 6857);
    }
}
