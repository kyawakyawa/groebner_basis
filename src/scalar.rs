use gmp::mpq::Mpq;
use gmp::mpz::Mpz;

pub type Integer = Mpz;
pub type Rational = Mpq;

pub fn gcd(a: &Integer, b: &Integer) -> Integer {
    if b == &Integer::zero() {
        a.clone()
    } else {
        let r = a % b;
        gcd(b, &r)
    }
}

pub fn lcm(a: &Integer, b: &Integer) -> Integer {
    a * b / gcd(a, b)
}
