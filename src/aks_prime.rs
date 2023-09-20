use num::integer::{gcd, Roots};
use std::vec;

#[derive(Debug)]
struct Polynomial {
    coef: Vec<u64>,
}

impl Polynomial {
    fn new() -> Self {
        Polynomial { coef: Vec::new() }
    }

    fn with_capacity(d: usize) -> Self {
        Polynomial {
            coef: Vec::with_capacity(d),
        }
    }

    fn get_coef_unchecked(&self, m: usize) -> u64 {
        unsafe { *self.coef.get_unchecked(m) }
    }

    #[allow(dead_code)]
    fn set_coef_unchecked(&mut self, m: usize, c: u64) {
        unsafe {
            *self.coef.get_unchecked_mut(m) = c;
        }
    }

    fn deg(&self) -> usize {
        self.coef.len() - 1
    }

    fn mod_mul(&self, q: &Polynomial, r: usize, n: u64) -> Polynomial {
        let max_deg;
        if self.deg() + q.deg() < r {
            max_deg = self.deg() + q.deg()
        } else {
            max_deg = r - 1;
        }
        let mut res = Polynomial::with_capacity(max_deg + 1);

        let mut coef;
        for i in 0..=max_deg {
            coef = 0;
            let jmin = if i > q.deg() { i - q.deg() } else { 0 };
            let jmax = if i < self.deg() { i } else { self.deg() };

            for j in jmin..=jmax {
                coef += self.get_coef_unchecked(j) * q.get_coef_unchecked(i - j) % n;
            }

            let jmin = i + r - q.deg();
            let jmax = self.deg();

            for j in jmin..=jmax {
                coef += self.get_coef_unchecked(j) * q.get_coef_unchecked(i + r - j) % n;
            }

            coef %= n;
            res.coef.push(coef);
        }
        res
    }

    // panics if n is zero
    fn mod_pow(&self, r: usize, n: u64) -> Polynomial {
        let mut res = Polynomial::new();
        res.coef.push(1);

        let mut i = n.ilog2() + 1;
        while i > 0 {
            res = res.mod_mul(&res, r, n);
            i -= 1;
            if n >> i & 1 == 1 {
                res = res.mod_mul(self, r, n);
            }
        }
        res
    }
}

impl PartialEq for Polynomial {
    fn eq(&self, other: &Self) -> bool {
        self.coef == other.coef
    }
}

impl From<Vec<u64>> for Polynomial {
    fn from(v: Vec<u64>) -> Self {
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
    let maxk = logn * logn;
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
        let mut xan = Polynomial::from(vec![a , 1]).mod_pow(r_ui as usize, n);
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

        assert_eq!(p.mod_pow(7, 6), vec![1, 0, 3, 2, 3, 0, 1].into());
        assert_eq!(p.mod_pow(8, 7), vec![1, 0, 0, 0, 0, 0, 0, 1].into());
    }
}
