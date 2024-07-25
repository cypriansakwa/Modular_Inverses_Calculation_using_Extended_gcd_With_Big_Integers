use num_bigint::BigInt;
use num_traits::{One, Zero};
use std::str::FromStr;

// Function to calculate the modular inverse
fn mod_inverse(a: &BigInt, m: &BigInt) -> Option<BigInt> {
    let (g, x, _) = extended_gcd(a.clone(), m.clone());
    if g.is_one() {
        Some((x % m + m) % m)  // Ensure the result is positive
    } else {
        None  // No inverse exists if gcd(a, m) != 1
    }
}

// Extended Euclidean Algorithm
fn extended_gcd(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    if b.is_zero() {
        (a, BigInt::one(), BigInt::zero())
    } else {
        let (g, x, y) = extended_gcd(b.clone(), a.clone() % b.clone());
        (g, y.clone(), x - (a / b) * y)
    }
} 

fn main() {
    let a = BigInt::from_str("12345678901234567890123456789012345678914242421").unwrap();
    let m = BigInt::from_str("9876543210987654321098765432109876543197772777771").unwrap();

    match mod_inverse(&a, &m) {
        Some(inv) => println!("The modular inverse is: {}", inv),
        None => println!("No modular inverse exists for the given input."),
    }
}


