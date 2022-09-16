// Problem id 3
// https://projecteuler.net/problem=3

// This calculates the largest prime of a number n
pub fn solution(mut n: u64) -> u64 {
    let mut primes: Vec<u64> = vec![2]; // List of primes calculated
    let mut current_prime = 2;

    // Would be dividing by primes until reaching 1
    while n != 1 { 
        // Dividing by current prime till it could
        if n % current_prime == 0 {
            n /= current_prime;
            continue;
        }

        // Finding the next prime
        else {
            'prime: loop {
                current_prime += 1; // Incrementing current_prime for trial as a prime

                // Trialing the current_prime with current known primes
                for prime in primes.iter() {
                    // If current_prime is divisible by any known primes, that means it's not a prime
                    if current_prime % prime == 0 {
                        continue 'prime;
                    }

                    // No need to trial for primes >= current_prime / 2
                    if current_prime / 2 >= *prime {
                        primes.push(current_prime);
                        break 'prime;
                    }
                }
            }
        }
    };

    current_prime
}

#[test]
fn testooooo() {
    assert_eq!(solution(13195), 29);
    assert_eq!(solution(600851475143), 6857);
}

