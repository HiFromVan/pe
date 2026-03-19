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
//! Trial division starting from 2. For each factor found, divide it out completely
//! before moving on — so any number that divides $n$ at this point must be prime
//! (all its smaller prime factors have already been removed).
//!
//! Only trial up to $\sqrt{n}$: if $n$ still has a factor, at least one must be
//! $\leq \sqrt{n}$. If $n > 1$ after the loop, the remainder is itself the largest prime factor.
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
