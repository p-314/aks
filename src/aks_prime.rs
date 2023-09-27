use num::integer::{gcd, Roots};
use std::vec;

use crate::traits::UInt;


//mod traits;

#[derive(Debug)]
struct Polynomial<I : UInt> {
    coef: Vec<I>,
}

impl<I: UInt> Polynomial<I> {
    fn new() -> Self {
        Polynomial { coef: Vec::new() }
    }

    fn with_capacity(d: usize) -> Self {
        Polynomial {
            coef: Vec::with_capacity(d),
        }
    }

    fn get_coef_unchecked(&self, m: usize) -> &I {
        unsafe { self.coef.get_unchecked(m) }
    }

    #[allow(dead_code)]
    fn set_coef_unchecked(&mut self, m: usize, c: I) {
        unsafe {
            *self.coef.get_unchecked_mut(m) = c;
        }
    }

    fn deg(&self) -> usize {
        self.coef.len() - 1
    }

    fn mod_mul(&self, q: &Polynomial<I>, r: usize, n: &I) -> Polynomial<I> {
        let max_deg;
        if self.deg() + q.deg() < r {
            max_deg = self.deg() + q.deg()
        } else {
            max_deg = r - 1;
        }
        let mut res = Polynomial::with_capacity(max_deg + 1);

        let mut coef;
        for i in 0..=max_deg {
            coef = I::zero();
            let jmin = if i > q.deg() { i - q.deg() } else { 0 };
            let jmax = if i < self.deg() { i } else { self.deg() };

            for j in jmin..=jmax {
                coef += self.get_coef_unchecked(j).mul(q.get_coef_unchecked(i - j));
            }

            let jmin = i + r - q.deg();
            let jmax = self.deg();

            for j in jmin..=jmax {
                coef += self.get_coef_unchecked(j).mul(q.get_coef_unchecked(i + r - j));
            }

            coef %= n;
            res.coef.push(coef);
        }
        res
    }

    // panics if n is zero
    fn mod_pow(&self, r: usize, n: &I) -> Polynomial<I> {
        let mut res = Polynomial::new();
        res.coef.push(I::one());

        let mut i = n.bits();
        while i > 0 {
            res = res.mod_mul(&res, r, n);
            i -= 1;
            if n.clone() >> i & I::one() == I::one() {
                res = res.mod_mul(self, r, n);
            }
        }
        res
    }
}

impl<I: UInt> PartialEq for Polynomial<I> {
    fn eq(&self, other: &Self) -> bool {
        self.coef == other.coef
    }
}

impl<I: UInt> From<Vec<I>> for Polynomial<I> {
    fn from(v: Vec<I>) -> Self {
        Polynomial { coef: v }
    }
}

fn is_perfect_power(n: u64) -> bool {
    if n == 0 || n == 1 {
        return true;
    }
    let max = n.ilog2();
    for i in 2..=max {
        if n.nth_root(i).pow(i) == n {
            return true;
        }
    }
    false
}

/// Returns `true` iff n is prime using the aks primality test
pub fn aks(n: u64) -> bool {
    if is_perfect_power(n) {
        return false;
    }

    let logn = n.ilog2() as u64 + 1;
    let lognf64 = (n as f64).log2();
    let maxk = (lognf64 * lognf64).ceil() as u64;
    let mut r_ui: u64 = 2;

    loop {
        let mut pow = 1;
        let mut found = true;
        for _k in 1..=maxk {
            pow *= n;
            pow %= r_ui;
            if pow == 1 {
                found = false;
                break;
            }
        }
        if found {
            break;
        }
        r_ui += 1;
    }
    println!("{}", r_ui);

    let mut a = r_ui;
    loop {
        if a < 2 {
            break;
        }
        let gcd = gcd(n, a);
        if 1 < gcd && gcd < n {
            return false;
        }
        a -= 1;
    }

    if n <= r_ui {
        return true;
    }

    let rsqrt = (r_ui as f32 - 1.0).sqrt() as u64 + 1;
    let maxa = rsqrt * logn;
    for a in 1..=maxa {
        let mut xan = Polynomial::from(vec![a , 1]).mod_pow(r_ui as usize, &n);
        let nmodr = (n % r_ui) as usize;
        if xan.deg() < nmodr {
            return false;
        }
        xan.coef[nmodr] -= 1;
        xan.coef[0] -= a % n;

        for c in xan.coef {
            if c % n != 0 {
                return false;
            }
        }
    }

    return true;
}

#[cfg(test)]
mod test {
    use crate::aks_prime::*;

    #[test]
    fn perfect_power() {
        assert!(is_perfect_power(0));
        assert!(is_perfect_power(1));
        assert!(is_perfect_power(4));
        assert!(is_perfect_power(8));
        assert!(is_perfect_power(9));
        assert!(is_perfect_power(128));

        assert!(!is_perfect_power(2));
        assert!(!is_perfect_power(24));
        assert!(!is_perfect_power(101));
    }

    #[test]
    fn mod_pow() {
        let p = Polynomial { coef: vec![1, 1] };

        assert_eq!(p.mod_pow(7, &6), vec![1, 0, 3, 2, 3, 0, 1].into());
        assert_eq!(p.mod_pow(8, &7), vec![1, 0, 0, 0, 0, 0, 0, 1].into());
    }

    fn trial_division(n: u64) -> bool {
        let nsqrt = (n as f64).sqrt().ceil() as u64;

        if n == 2 {
            return true;
        }
        if n % 2 == 0 {
            return false;
        }

        for d in 3..=nsqrt {
            if n % d == 0 {
                return false;
            }
        }
        true
    }

    #[test]
    fn small() {
        for n in 2..10000 {
            assert_eq!(aks(n), trial_division(n));
        }
    }
}
