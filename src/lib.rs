//! # Project Euler Solutions
//!
//! This crate contains solutions to [Project Euler](https://projecteuler.net/) problems.
//!
//! Each problem is implemented as a separate binary in the `problems/` directory.
//!
//! ## Usage
//!
//! Run a specific problem:
//! ```bash
//! cargo run --bin 001
//! ```
//!
//! Run tests for a problem:
//! ```bash
//! cargo test --bin 001
//! ```
//!
//! Generate documentation with rendered math formulas:
//! ```bash
//! cargo doc --open
//! ```

/// Common mathematical utilities for Project Euler problems
pub mod math {
    /// Check if a number is prime
    pub fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }
        let limit = (n as f64).sqrt() as u64;
        for i in (3..=limit).step_by(2) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }

    /// Sieve of Eratosthenes: returns a boolean vec where index i is true if i is prime.
    /// `sieve[0]` and `sieve[1]` are false; valid for indices 0..=limit.
    pub fn sieve_of_eratosthenes(limit: usize) -> Vec<bool> {
        let mut is_prime = vec![true; limit + 1];
        is_prime[0] = false;
        if limit > 0 {
            is_prime[1] = false;
        }
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
        is_prime
    }

    /// Calculate greatest common divisor using Euclidean algorithm
    pub fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    /// Calculate least common multiple
    pub fn lcm(a: u64, b: u64) -> u64 {
        a * b / gcd(a, b)
    }

    /// Arithmetic series sum: a + (a+d) + (a+2d) + ... (n terms)
    /// = n/2 * (2a + (n-1)d)
    pub fn arithmetic_series_sum(a: i64, d: i64, n: u64) -> i64 {
        let n = n as i64;
        n * (2 * a + (n - 1) * d) / 2
    }

    /// Bernoulli numbers B_0..=B_max as exact fractions (numerator, denominator).
    /// Uses the recurrence: sum_{k=0}^{m} C(m+1,k) * B_k = 0  (m >= 1), B_0 = 1.
    fn bernoulli_numbers(max: usize) -> Vec<(i64, i64)> {
        // Store as (num, den) in lowest terms
        let mut b: Vec<(i64, i64)> = vec![(0, 1); max + 1];
        b[0] = (1, 1);

        for m in 1..=max {
            // B_m = -1/(m+1) * sum_{k=0}^{m-1} C(m+1, k) * B_k
            let mut num: i64 = 0;
            let mut den: i64 = 1;
            let mut c = 1i64; // C(m+1, k)
            for k in 0..m {
                // accumulate C(m+1,k) * B_k into num/den
                let (bn, bd) = b[k];
                // add c * bn/bd to num/den
                let new_num = num * bd + c * bn * den;
                let new_den = den * bd;
                let g = gcd(new_num.unsigned_abs(), new_den.unsigned_abs() as u64) as i64;
                num = new_num / g;
                den = new_den / g;
                // update binomial coefficient: C(m+1,k+1) = C(m+1,k)*(m+1-k)/(k+1)
                c = c * (m as i64 + 1 - k as i64) / (k as i64 + 1);
            }
            // B_m = -num / ((m+1) * den)
            let result_num = -num;
            let result_den = (m as i64 + 1) * den;
            let g = gcd(result_num.unsigned_abs(), result_den.unsigned_abs() as u64) as i64;
            b[m] = (result_num / g, result_den / g);
        }
        b
    }

    /// Sum of p-th powers: 1^p + 2^p + ... + n^p, using Bernoulli numbers.
    /// Returns exact i64 result (assumes it fits).
    pub fn power_sum(n: u64, p: usize) -> i64 {
        // The Bernoulli formula computes sum_{k=0}^{m-1} k^p when passed m.
        // To get sum_{k=1}^{n} k^p we pass m = n+1.
        let m = n + 1;
        // Formula: 1/(p+1) * sum_{j=0}^{p} C(p+1,j) * B_j * m^(p+1-j)
        let b = bernoulli_numbers(p);
        let mut num: i64 = 0;
        let mut den: i64 = 1;
        let mut c = 1i64; // C(p+1, j)
        for j in 0..=p {
            let (bj_n, bj_d) = b[j];
            let n_pow = (m as i64).pow((p + 1 - j) as u32);
            // term = c * bj_n/bj_d * n_pow
            let term_num = c * bj_n * n_pow;
            let term_den = bj_d;
            let new_num = num * term_den + term_num * den;
            let new_den = den * term_den;
            let g = gcd(new_num.unsigned_abs(), new_den.unsigned_abs() as u64) as i64;
            num = new_num / g;
            den = new_den / g;
            // C(p+1, j+1) = C(p+1,j) * (p+1-j) / (j+1)
            if j < p {
                c = c * (p as i64 + 1 - j as i64) / (j as i64 + 1);
            }
        }
        // divide by (p+1)
        num / ((p as i64 + 1) * den)
    }
}
