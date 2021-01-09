mod degree;
mod monomial;
mod polynomial;
mod scalar;

use degree::Zn;
use monomial::Monomial;
use scalar::Integer;
use std::time::Instant;

fn factorial(n: &Integer) -> Integer {
    if n == &Integer::zero() {
        Integer::one()
    } else {
        n * &factorial(&(n - &Integer::one()))
    }
}

fn main() {
    let start = Instant::now();
    for _i in 0..101 {
        println!("{}!={}", _i, factorial(&Integer::from(_i)));
    }
    let end = start.elapsed();

    println!("{}.{:03}sec", end.as_secs(), end.subsec_nanos() / 1_000_000);

    let z0 = Zn::from(vec![Integer::from(0), Integer::from(1), Integer::from(2)]);
    let z1 = Zn::from(vec![Integer::from(3), Integer::from(4), Integer::from(5)]);
    let z2 = z0.clone();

    println!("{}", &z0 + &z1);
    println!("{:?}", &z0 - &z1);
    println!("{:?}", z2);

    let x = Monomial::from(vec![Integer::from(0), Integer::from(1), Integer::from(2)]);
    let y = Monomial::from(vec![Integer::from(3), Integer::from(4), Integer::from(5)]);

    println!("{}", x);
    println!("{:?}", y);
    println!("{}", &x * &y);
    println!("{}", &y / &x);
    println!("{:?}", x.cmp(&y));
}
