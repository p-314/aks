use rug::{Integer, integer::IntegerExt64};
use std::vec;

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

    fn set_coef_unchecked(&mut self, m: usize, c: u64) {
        unsafe { 
            *self.coef.get_unchecked_mut(m) = c;
        }
    }

    fn monom(d: usize) -> Self {
        let mut c = Vec::with_capacity(d);
        for _i in 0..d {
            c.push(0);
        }
        c.push(1);
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

    fn mod_mul(&self, q: &Polynomial, r: usize, n: u64) -> Polynomial {
        let max_deg;
        if self.deg() + q.deg() < r {
            max_deg = self.deg() + q.deg()
        } else {
            max_deg = r - 1;
        }
        let mut res = Polynomial::with_capacity(max_deg);

        for i in 0..r {
            let mut coef = 0;

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

    fn mod_pow(&self, r: usize, n: u64) -> Polynomial {
        let mut res = Polynomial::new();
        res.coef.push(1);

        let mut i = (n as f64).log2() as u64 + 1;
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

fn aks(n: u64) -> bool {
    if Integer::from(n).is_perfect_power() {
        return false;
    }

    let logn = (n as f64).log2() as u64;
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
        let gcd =Integer::from(n).gcd_u64(a).to_u64().unwrap();
        if 1 < gcd && &gcd < &n {
            return false;
        }
        a -= 1;
    }

    if n <= r_ui {
        return true;
    }

    //Step 5
    let rsqrt = (r_ui as f32 - 1.0).sqrt() as u64 + 1;
    let maxa = rsqrt * logn;
    for a in 1..=maxa {
        let mut xan = (Polynomial {
            coef: vec![a as u64, 1],
        })
        .mod_pow(r_ui as usize, n);
        let nmodr = (n % r_ui) as usize;
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
    use std::{fs, str::FromStr, time::Instant};

    use super::aks;

    pub fn test1() {
        let start = Instant::now();
        let mut primes = 0;
        for n in 1000000..1000100 {
            let prime = aks(n);
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
            let n = u64::from_str(line.trim()).unwrap();
            let now = Instant::now();
            let prime = aks(n);
            let time = now.elapsed();
            output += &format!("{}\n", time.as_micros() as f64 / 1_000_000.0);

            println!("{} {} {:?}", n, prime, time);
        }

        fs::write("times.txt", output).unwrap();
    }

    pub fn test3() {
        let start = Instant::now();
        let prime = "31";
        let is_prime = aks(u64::from_str(prime).unwrap());
        println!("{} {}", prime, is_prime);
        println!("{} {:?}", is_prime, start.elapsed())
    }
}

fn main() {
    test::test3();
}
