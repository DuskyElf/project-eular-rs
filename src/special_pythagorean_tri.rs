// Problem id 9
// https://projecteuler.net/problem=9

// This finds the exactly one pythagorean triplet for which a + b + c == n
// returns a * b * c if found
pub fn solution(target: i32) -> Option<i32> {
    // Brute forcing all possible combination untill found the appropriate
    // Starting from target / 2, because it never gonna be greater than that
    let mut a = target / 2;
    while a > 0 {
        let mut b = target / 2;
        while b > 0 {
            let mut c = target / 2;
            while c > 0 {
                if a + b + c == target {
                    if is_pythagorean(a, b, c) {
                        return Some(a * b * c);
                    }
                }

                c -= 1;
            }
            b -= 1;
        }
        a -= 1;
    }
    None
}

// All pythagorean triplets of a, b, c states a^2 + b^2 = c^2
fn is_pythagorean(a: i32, b: i32, c: i32) -> bool {
    return a * a + b * b == c * c;
}

#[test]
fn testoo() {
    assert_eq!(solution(1000).unwrap(), 31875000);
}
