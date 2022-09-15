// Problem id 1
// https://projecteuler.net/problem=1

// This calculates the sum of all multiples of 3 or 5 between 1..n
pub fn solution(n: u32) -> u32 {
    let mut result = 0;
    for num in 1..n {
        if num % 3 == 0 || num % 5 == 0 {
            result += num
        }
    }

    result
}

#[test]
fn multiples_of_3_or_5() {
    assert_eq!(solution(10), 23);
    assert_eq!(solution(1000), 233168);
}

