//! # Problem 7: 10001st Prime
//!
//! **Difficulty:** ⭐
//! **Link:** <https://projecteuler.net/problem=7>
//!
//! ## Problem Statement
//!
//! By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see
//! that the 6th prime is 13.
//!
//! What is the 10001st prime number?
//!
//! ## Mathematical Solution
//!
//! **暴力法** `solve`：无限迭代器逐个 `is_prime` 检查，O(n√n)。
//!
//! **埃式筛法** `solve1`：
//! 由质数定理，第 n 个素数 $p_n \approx n(\ln n + \ln \ln n)$，
//! 对 n ≥ 6 这是严格上界，用它确定筛的范围后一次埃式筛出所有素数。
//!
//! ## Complexity
//!
//! - `solve`:  Time $O(n \sqrt{n})$, Space $O(1)$
//! - `solve1`: Time $O(m \log \log m)$, Space $O(m)$，其中 $m \approx n \ln n$

use pe::math::is_prime;

#[allow(dead_code)]
fn solve() -> u64 {
    (2u64..).filter(|&n| is_prime(n)).nth(10000).unwrap()
}

/// 埃式筛法：用质数定理估算上界，一次筛出所有素数
fn solve1() -> u64 {
    let n = 10001usize;
    // 质数定理上界：p_n < n*(ln n + ln ln n)，n >= 6 时严格成立
    let ln_n = (n as f64).ln();
    let limit = (n as f64 * (ln_n + ln_n.ln())).ceil() as usize + 10;

    // 埃式筛
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= limit {
        if is_prime[i] {
            let mut j = i * i;
            while j <= limit {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    is_prime.iter().enumerate()
        .filter(|&(_, &p)| p)
        .map(|(i, _)| i as u64)
        .nth(n - 1)
        .unwrap()
}

fn main() {
    println!("{}", solve1());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        use pe::math::is_prime;
        let sixth = (2u64..).filter(|&n| is_prime(n)).nth(5).unwrap();
        assert_eq!(sixth, 13);
    }

    #[test]
    fn test_answer() {
        assert_eq!(solve(), 104743);
        assert_eq!(solve1(), 104743);
    }
}
