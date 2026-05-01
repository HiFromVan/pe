//! # Problem 9: Special Pythagorean Triplet
//!
//! **Difficulty:** ⭐
//! **Link:** <https://projecteuler.net/problem=9>
//!
//! ## Problem Statement
//!
//! A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
//! a² + b² = c²
//!
//! For example, 3² + 4² = 9 + 16 = 25 = 5².
//!
//! There exists exactly one Pythagorean triplet for which a + b + c = 1000.
//! Find the product abc.
//!
//! ## Mathematical Solution
//!
//! 已知 a² + b² = c²，a + b + c = 1000，求 a, b, c。
//!
//! **方法一：枚举**
//! 由 c = 1000 - a - b 代入勾股条件：
//! a² + b² = (1000 - a - b)²
//! 枚举 a ∈ [1, 333]，b ∈ [a+1, (1000-a)/2]，验证等式。
//!
//! **方法二：欧几里得参数化公式**
//! 所有本原勾股数由互质的 m > n > 0 且 m-n 为奇数生成：
//! a = k(m²-n²), b = k(2mn), c = k(m²+n²)
//!
//! 代入 a+b+c = 1000：
//! k · 2m(m+n) = 1000  →  k = 500 / (m(m+n))
//!
//! 枚举满足条件的 m, n，找到整除的 k 即得解。
//!
//! ## Complexity
//!
//! - `solve`:  Time $O(n^2)$, Space $O(1)$
//! - `solve1`: Time $O(\sqrt{n})$, Space $O(1)$

use pe::math::gcd;

fn solve(sum: u64) -> u64 {
    for a in 1..sum / 3 {
        for b in (a + 1)..(sum - a) / 2 + 1 {
            let c = sum - a - b;
            if a * a + b * b == c * c {
                return a * b * c;
            }
        }
    }
    0
}

fn solve1(sum: u64) -> u64 {
    // k * 2m(m+n) = sum  =>  k = sum / (2m(m+n))
    let half = sum / 2;
    let mut m = 2;
    while m * m < sum {
        if half % m == 0 {
            let half_m = half / m;
            // 找 n：n < m, gcd(m,n)=1, m-n 奇数, (m+n) | half_m
            let mut n = 1 + (m % 2); // 保证 m-n 为奇数
            while n < m {
                if (m - n) % 2 == 1 && gcd(m, n) == 1 && half_m % (m + n) == 0 {
                    let k = half_m / (m + n);
                    let a = k * (m * m - n * n);
                    let b = k * 2 * m * n;
                    let c = k * (m * m + n * n);
                    return a * b * c;
                }
                n += 2;
            }
        }
        m += 1;
    }
    0
}

fn main() {
    println!("{}", solve1(1000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // 3² + 4² = 5², 3+4+5 = 12
        assert_eq!(solve(12), 60);
        assert_eq!(solve1(12), 60);
    }

    #[test]
    fn test_answer() {
        assert_eq!(solve(1000), 31875000);
        assert_eq!(solve1(1000), 31875000);
    }
}
