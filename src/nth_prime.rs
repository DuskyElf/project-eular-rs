// Problem id 7
// https://projecteuler.net/problem=7

// This finds the nth prime number
pub fn solution(n: u64) -> u64 {
    // First prime in 2
    if n == 1 {
        return 2;
    }

    let mut primes = vec![2];
    let mut prime = 2;
    let mut trial = 3;

    // Trialing each number to be prime untill finding the nth prime
    let mut i = 0;
    while i != n {
        if new_is_prime(trial, &mut primes) {
            prime = trial;
            i += 1;
        }

        trial += 1;
    }

    prime
}

fn new_is_prime(n: u64, primes: &mut Vec<u64>) -> bool {
    // Checking for each previous prime as it's factor
    // till n / 2, because prime factors of n never gonna be higher than n / 2
    for prime in primes.iter() {
        if *prime >= n / 2 {
            break;
        }

        if n % *prime == 0 {
            return false;
        }
    }

    primes.push(n);
    true
}

#[test]
fn testoooooooooooooooo() {
    assert_eq!(solution(6), 13);
    assert_eq!(solution(8), 19);
    // assert_eq!(solution(10001), 104743);
}
