use rug::Integer;
use std::vec;

struct Polynomial {
    coef: Vec<Integer>,
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

    fn get_coef_unchecked(&self, m: usize) -> &Integer {
        unsafe { self.coef.get_unchecked(m) }
    }

    fn set_coef_unchecked(&mut self, m: usize, c: Integer) {
        unsafe { 
            *self.coef.get_unchecked_mut(m) = c;
        }
    }

    fn monom(d: usize) -> Self {
        let mut c = Vec::with_capacity(d);
        for _i in 0..d {
            c.push(Integer::from(0));
        }
        c.push(Integer::from(1));
        Polynomial { coef: c }
    }

    fn deg(&self) -> usize {
        self.coef.len() - 1
    }

    fn trim(&mut self) {
        let mut max_deg = self.deg();
        for i in (0..=self.deg()).rev() {
            if self.coef[i] == 0 {
                max_deg -= 1;
            } else {
                break;
            }
        }
        self.coef = self.coef[..=max_deg].to_vec();
    }

    fn mod_mul(&self, q: &Polynomial, r: usize, n: &Integer) -> Polynomial {
        let max_deg;
        if self.deg() + q.deg() < r {
            max_deg = self.deg() + q.deg()
        } else {
            max_deg = r - 1;
        }
        let mut res = Polynomial::with_capacity(max_deg);

        for i in 0..r {
            let mut coef = Integer::from(0);

            let jmin = if i > q.deg() { i - q.deg() } else { 0 };
            let jmax = if i < self.deg() { i } else { self.deg() };

            for j in jmin..=jmax {
                coef += self.get_coef_unchecked(j) * q.get_coef_unchecked(i - j);
            }

            let jmin = i + r - q.deg();
            let jmax = self.deg();

            for j in jmin..=jmax {
                coef += self.get_coef_unchecked(j) * q.get_coef_unchecked(i + r - j);
            }

            coef %= n;
            res.coef.push(coef);
        }
        res
    }

    fn mod_pow(&self, r: usize, n: &Integer) -> Polynomial {
        let mut res = Polynomial::new();
        res.coef.push(Integer::from(1));

        let mut i = n.significant_bits() + 1;
        while i > 0 {
            res = res.mod_mul(&res, r, n);
            i -= 1;
            if n.get_bit(i) {
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

fn aks(n: &Integer) -> bool {
    if n.is_perfect_power() {
        return false;
    }

    let logn = n.significant_bits();
    let maxk = logn * logn;
    let mut r_ui: usize = 2;

    loop {
        let mut pow = Integer::from(1);
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
    let r = Integer::from(r_ui);
    //println!("{}", r);

    let mut a = r.clone();
    loop {
        if a < 2 {
            break;
        }
        let gcd: Integer = n.gcd_ref(&a).into();
        if 1 < gcd && &gcd < n {
            return false;
        }
        a -= 1;
    }

    if n <= &r_ui {
        return true;
    }

    //Step 5
    let rsqrt = (r_ui as f32 - 1.0).sqrt() as u32 + 1;
    let maxa = rsqrt * logn;
    for a in 1..=maxa {
        let mut xan = (Polynomial {
            coef: vec![Integer::from(a), Integer::from(1)],
        })
        .mod_pow(r_ui, &n);
        let nmodr = n.mod_u(r_ui as u32) as usize;
        if xan.deg() < nmodr {
            return false;
        }
        xan.coef[nmodr] -= 1;
        xan.coef[0] -= a;

        for c in xan.coef {
            if c % n != 0 {
                return false;
            }
        }
    }

    return true;
}

pub mod test {
    use rug::Integer;
    use std::{fs, str::FromStr, time::Instant};

    use super::aks;

    pub fn test1() {
        let start = Instant::now();
        let mut primes = 0;
        for n in 1000000..1000100 {
            let prime = aks(&Integer::from(n));
            if prime {
                primes += 1;
            }
            println!("{n} {}", prime);
        }
        println!("{} {:?}", primes, start.elapsed())
    }

    pub fn test2() {
        let file = fs::read_to_string("src/test.txt").unwrap();
        let mut output = String::new();

        for line in file.split("\n") {
            let n = Integer::from_str(line).unwrap();
            let now = Instant::now();
            let prime = aks(&n);
            let time = now.elapsed();
            output += &format!("{}\n", time.as_micros() as f64 / 1_000_000.0);

            println!("{} {} {:?}", n, prime, time);
        }

        fs::write("times.txt", output).unwrap();
    }

    pub fn test3() {
        let start = Instant::now();
        let prime = "86028121";
        let is_prime = aks(&Integer::from_str(prime).unwrap());
        println!("{} {}", prime, is_prime);
        println!("{} {:?}", is_prime, start.elapsed())
    }
}

fn main() {
    test::test3();
}
