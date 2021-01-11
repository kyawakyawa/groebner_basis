mod degree;
mod groebner_basis;
mod monomial;
mod polynomial;
mod scalar;

use degree::Zn;
use monomial::Monomial;
use polynomial::{Polynomial, PolynomialHandlers};
use scalar::{Integer, Rational};
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

    let mut f = Polynomial::from(2);
    f.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(2), Integer::from(1)]),
    );
    f.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(1), Integer::from(2)]),
    );
    f.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(0), Integer::from(2)]),
    );

    let mut f1 = Polynomial::from(2);
    f1.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(0), Integer::from(2)]),
    );
    f1.add_term(
        Rational::from(-1),
        Monomial::from(vec![Integer::from(0), Integer::from(0)]),
    );

    let mut f2 = Polynomial::from(2);
    f2.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(1), Integer::from(1)]),
    );
    f2.add_term(
        Rational::from(-1),
        Monomial::from(vec![Integer::from(0), Integer::from(0)]),
    );

    let (a, r) = f.polynomial_divide(&vec![f1, f2]);

    println!("fin");
    for p in a {
        println!("{}", p);
    }
    println!("{}", r);

    // s polynomial
    let mut f = Polynomial::from((2, monomial::MonomialOrder::Grlex));
    f.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(3), Integer::from(2)]),
    );
    f.add_term(
        Rational::from(-1),
        Monomial::from(vec![Integer::from(2), Integer::from(3)]),
    );
    f.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(1), Integer::from(0)]),
    );

    let mut g = Polynomial::from((2, monomial::MonomialOrder::Grlex));
    g.add_term(
        Rational::from(3),
        Monomial::from(vec![Integer::from(4), Integer::from(1)]),
    );
    g.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(0), Integer::from(2)]),
    );

    let s_fg = polynomial::s_polynomial(&f, &g);

    match s_fg {
        Some(s_fg) => {
            println!("S({}, {}) = {}", f, g, s_fg);
        }
        None => {
            println!("failed");
        }
    }

    // groebner basis
    let mut f0 = Polynomial::from((3, monomial::MonomialOrder::Lex));
    f0.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(2), Integer::from(0), Integer::from(0)]),
    );
    f0.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(0), Integer::from(2), Integer::from(0)]),
    );
    f0.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(0), Integer::from(0), Integer::from(2)]),
    );
    f0.add_term(
        Rational::from(-1),
        Monomial::from(vec![Integer::from(0), Integer::from(0), Integer::from(0)]),
    );

    let mut f1 = Polynomial::from((3, monomial::MonomialOrder::Lex));
    f1.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(2), Integer::from(0), Integer::from(0)]),
    );
    f1.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(0), Integer::from(0), Integer::from(2)]),
    );
    f1.add_term(
        Rational::from(-1),
        Monomial::from(vec![Integer::from(0), Integer::from(1), Integer::from(0)]),
    );

    let mut f2 = Polynomial::from((3, monomial::MonomialOrder::Lex));
    f2.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(1), Integer::from(0), Integer::from(0)]),
    );
    f2.add_term(
        Rational::from(-1),
        Monomial::from(vec![Integer::from(0), Integer::from(0), Integer::from(1)]),
    );

    let fs = vec![f0, f1, f2];
    println!("compute groebner basis");
    for f in fs.iter() {
        println!("{}", f);
    }

    let gb = groebner_basis::compute_groebner_basis(fs);

    println!("fin");
    for g in gb {
        println!("{}", g);
    }
}
