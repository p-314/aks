mod aks_prime;

pub mod test {
    use std::{fs, str::FromStr, time::Instant};
    use crate::aks_prime::aks;

    pub fn test_interval_big() {
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

    pub fn test_small() {
        let file = fs::read_to_string("data/test.txt").unwrap();
        let mut output = String::new();

        for line in file.split("\n") {
            let n = u64::from_str(line.trim()).unwrap();
            let now = Instant::now();
            let prime = aks(n);
            let time = now.elapsed();
            output += &format!("{}\n", time.as_micros() as f64 / 1_000_000.0);

            println!("{} {} {:?}", n, prime, time);
        }

        fs::write("data/times_small.txt", output).unwrap();
    }

    pub fn test_single_big() {
        let start = Instant::now();
        let prime = "4093082899";
        let is_prime = aks(u64::from_str(prime).unwrap());
        println!("{} {}", prime, is_prime);
        println!("{} {:?}", is_prime, start.elapsed())
    }
}

fn main() {
    //test::test_small();
    test::test_interval_big();
    //test::test_single_big();
}
