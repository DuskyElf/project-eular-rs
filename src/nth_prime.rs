// Problem id 7
// https://projecteuler.net/problem=7

pub fn solution(n: u64) -> u64 {
    if n == 1 {
        return 2;
    }

    let mut primes = vec![2];
    let mut prime = 2;
    let mut trial = 3;

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
