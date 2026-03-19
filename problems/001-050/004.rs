//! # Problem 4: Largest Palindrome Product
//!
//! **Difficulty:** ⭐
//! **Link:** <https://projecteuler.net/problem=4>
//!
//! ## Problem Statement
//!
//! A palindromic number reads the same both ways. The largest palindrome made
//! from the product of two 2-digit numbers is 9009 = 91 × 99.
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.
//!
//! ## Mathematical Solution
//!
//! TODO
//!
//! ## Complexity
//!
//! - Time: $O(?)$
//! - Space: $O(?)$

fn solve() -> u64 {
    let mut max_palindrome = 0u64;
    for i in (100u64..=999).rev() {
        if i * 999 < max_palindrome {
            break;
        }
        for j in (100u64..=999).rev() {
            let product = i * j;
            if product < max_palindrome {
                break;
            }
            if is_palindrome(product) {
                max_palindrome = product;
                break;
            }
        }
    }
    max_palindrome
}

fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

fn main() {
    let answer = solve();
    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(solve(), 906609);
    }
}
