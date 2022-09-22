// Problem id 10
// https://projecteuler.net/problem=10

// Re-using function from problem 7
use crate::nth_prime::new_is_prime;

// This calculates the sum of all the primes below n
pub fn solution(n: u64) -> u64{
    let mut result = 2;

    if n == 3 {
        return result;
    }

    let mut primes = vec![2];

    // Adding each prime number below n to result
    for i in 3..n {
        if new_is_prime(i, &mut primes) {
            result += i;
        }
    }

    result
}

#[test]
fn testoooooo() {
    assert_eq!(solution(10), 17);
    // assert_eq!(solution(2000000), 142913828922);
}
