pub fn trial_division(n: u64) -> bool {
    if n < 4 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let nsqrt = (n as f64).sqrt().ceil() as u64;

    let mut d = 3;
    while d <= nsqrt {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

#[cfg(test)]
pub mod test {
    use crate::aks_prime::aks;
    use crate::utils::trial_division;

    #[test]
    fn small() {
        for n in 2..10000 {
            assert_eq!(aks(n), trial_division(n));
        }
    }
}

pub mod timing {
    use crate::{aks_prime::aks, utils::trial_division};
    use std::{fs, str::FromStr, time::Instant};

    pub fn test_interval_big() {
        let start = Instant::now();
        let mut primes = 0;
        for n in 1000000..1001000 {
            let prime = aks(n);
            if prime {
                primes += 1;
            }
            println!("{n} {}", prime);
        }
        println!("{} {:?}", primes, start.elapsed())
    }

    pub fn test_small() {
        let file = fs::read_to_string("testing/small/test.txt").unwrap();
        let mut output = String::new();

        for line in file.split("\n") {
            let n = u64::from_str(line.trim()).unwrap();
            let now = Instant::now();
            let prime = aks(n);
            let time = now.elapsed();
            output += &format!("{}\n", time.as_micros() as f64 / 1_000_000.0);

            println!("{} {} {:?}", n, prime, time);
        }

        fs::write("testing/small/times_small.txt", output).unwrap();
    }

    pub fn test_log() {
        let file_p = fs::read_to_string("testing/prim_log.txt").unwrap();
        let mut output = String::new();

        for line in file_p.split("\n") {
            if let Ok(n) = u64::from_str(line.trim()) {
                let now = Instant::now();
                let prime = aks(n);
                let time = now.elapsed();
                output += &format!("{}\n", time.as_micros() as f64 / 1_000_000.0);

                println!("{} {} {:?}", n, prime, time);
            }
        }

        fs::write("testing/prim_log/times_prim_log.txt", output).unwrap();

        let file_c = fs::read_to_string("data/comp_log.txt").unwrap();
        let mut output = String::new();

        for line in file_c.split("\n") {
            if let Ok(n) = u64::from_str(line.trim()) {
                let now = Instant::now();
                let prime = aks(n);
                let time = now.elapsed();
                output += &format!("{}\n", time.as_micros() as f64 / 1_000_000.0);

                println!("{} {} {:?}", n, prime, time);
            }
        }

        fs::write("testing/comp_log/times_comp_log.txt", output).unwrap();
    }

    pub fn test_prim_log_int() {
        let file_p = fs::read_to_string("testing/prim_log_int/prim_log_int.txt").unwrap();
        let mut output = String::new();

        for line in file_p.split("\n") {
            if let Ok(n) = u64::from_str(line.trim()) {
                let now = Instant::now();
                let prime = aks(n);
                let time = now.elapsed();
                output += &format!("{}\n", time.as_micros() as f64 / 1_000_000.0);

                println!("{} {} {:?}", n, prime, time);
            }
        }

        fs::write("testing/prim_log_int/times_prim_log_int.txt", output).unwrap();
    }

    pub fn test_single_big() {
        let start = Instant::now();
        let prime = "104395301";
        let is_prime = aks(u64::from_str(prime).unwrap());
        println!("{} {}", prime, is_prime);
        println!("{} {:?}", is_prime, start.elapsed())
    }

    pub fn test_sieve() {
        let file_p = fs::read_to_string("testing/big/big.txt").unwrap();
        let mut output = String::new();
        let mut constant = Vec::new();

        for line in file_p.split("\n") {
            if let Ok(n) = u64::from_str(line.trim()) {
                let now = Instant::now();
                let prime = trial_division(n);
                let time = now.elapsed().as_micros() as f64 / 1_000_000.;

                output += &format!("{}\n", time);
                constant.push(time / (n as f64).sqrt());

                println!("{} {} {:?}", n, prime, time);
            }
        }

        println!("c: {}", constant.iter().sum::<f64>() / constant.len() as f64);

        fs::write("testing/prim_log_sieve/times_prim_log_sieve.txt", output).unwrap();
    }
}
