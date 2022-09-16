// Problem id 4
// https://projecteuler.net/problem=4

// This calculates the the largest palindrome made from the product of two 3 digit numbers
pub fn solution() -> u32 {
    let mut result = 0;

    // Checking for all 3 digit multiples for 11 number's product
    // because the palindrome can be written as abccba, as first digit is equal to last one anb so on
    // which then simplifies to:
    // => 100000a + 10000b + 1000c + 100c + 10b + a
    // => 100001a + 10010b + 1100c
    // => 11 (9091a + 910b + 100c)
    // So the palindrome must be divisible by 11
    let mut i = 999;
    while i > 100 {
        let mut j = 990;
        while j > 100 {
            let product = i * j;
            if product > result && is_palindrome(product) {
                // If new palindrome is bigger than the previous one, then save it as new one
                result = product;
            }

            // Seeing that 11 is prime, at least of of the factors of that palindrome must be divisible by 11
            j -= 11;
        }

        i -= 1;
    }
    
    result
}

fn is_palindrome(n: u32) -> bool {
    // If it's reverse is same as itself, it's a palindrome
    n.to_string() == n.to_string().chars().rev().collect::<String>()
}

#[test]
fn largest_palindrome_pro() {
    assert_eq!(solution(), 906609);
}

