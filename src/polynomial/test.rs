#[cfg(test)]
#[allow(unused_imports)]
use super::{s_polynomial, Polynomial, PolynomialHandlers};
#[allow(unused_imports)]
use crate::monomial;
#[allow(unused_imports)]
use crate::monomial::{Monomial, MonomialHandlers};
#[allow(unused_imports)]
use crate::scalar::{Integer, Rational};

#[test]
fn test_polynomial_divide() {
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

    let mut f0 = Polynomial::from(2);
    f0.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(0), Integer::from(2)]),
    );
    f0.add_term(
        Rational::from(-1),
        Monomial::from(vec![Integer::from(0), Integer::from(0)]),
    );

    let mut f1 = Polynomial::from(2);
    f1.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(1), Integer::from(1)]),
    );
    f1.add_term(
        Rational::from(-1),
        Monomial::from(vec![Integer::from(0), Integer::from(0)]),
    );

    let (a, r) = f.polynomial_divide(&vec![f0.clone(), f1.clone()]);

    let correct = &a[0] * &f0 + &a[1] * &f1 + &r;
    assert_eq!(f, correct);

    let lm_r = r.fetch_lm();
    let lm_f0 = f0.fetch_lm();
    let lm_f1 = f1.fetch_lm();

    match (lm_r, lm_f0, lm_f1) {
        (Some(lm_r), Some(lm_f0), Some(lm_f1)) => {
            assert!(!lm_r.is_divisible_by(&lm_f0));
            assert!(!lm_r.is_divisible_by(&lm_f1));
        }
        (_, _, _) => {
            panic!("failed to calc lm");
        }
    }

    // a0
    let mut correct = Polynomial::from(2);
    correct.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(1), Integer::from(0)]),
    );
    correct.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(0), Integer::from(0)]),
    );
    assert_eq!(a[0], correct);

    // a1
    let mut correct = Polynomial::from(2);
    correct.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(1), Integer::from(0)]),
    );
    assert_eq!(a[1], correct);

    // r
    let mut correct = Polynomial::from(2);
    correct.add_term(
        Rational::from(2),
        Monomial::from(vec![Integer::from(1), Integer::from(0)]),
    );
    correct.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(0), Integer::from(0)]),
    );
    assert_eq!(r, correct);
}

#[test]
fn test_s_polynomial() {
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

    let s_fg = s_polynomial(&f, &g);

    match s_fg {
        Some(s_fg) => {
            println!("S({}, {}) = {}", f, g, s_fg);

            let mut correct = Polynomial::from((2, monomial::MonomialOrder::Grlex));

            correct.add_term(
                Rational::from(-1),
                Monomial::from(vec![Integer::from(3), Integer::from(3)]),
            );
            correct.add_term(
                Rational::from(1),
                Monomial::from(vec![Integer::from(2), Integer::from(0)]),
            );
            correct.add_term(
                &Rational::from(-1) / &Rational::from(3),
                Monomial::from(vec![Integer::from(0), Integer::from(3)]),
            );
            println!("correct {}", correct);

            assert_eq!(s_fg, correct);
        }
        None => {
            panic!("Failed to compute s polynomial.");
        }
    }
}

#[test]
fn test_normalize() {
    let mut f = Polynomial::from((2, monomial::MonomialOrder::Grlex));
    f.add_term(
        Rational::from(7),
        Monomial::from(vec![Integer::from(3), Integer::from(2)]),
    );
    f.add_term(
        Rational::from(-5),
        Monomial::from(vec![Integer::from(2), Integer::from(3)]),
    );
    f.add_term(
        Rational::from(3),
        Monomial::from(vec![Integer::from(1), Integer::from(0)]),
    );

    let f = f.normalize();

    let mut correct = Polynomial::from((2, monomial::MonomialOrder::Grlex));
    correct.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(3), Integer::from(2)]),
    );
    correct.add_term(
        Rational::from(-5) / Rational::from(7),
        Monomial::from(vec![Integer::from(2), Integer::from(3)]),
    );
    correct.add_term(
        Rational::from(3) / Rational::from(7),
        Monomial::from(vec![Integer::from(1), Integer::from(0)]),
    );

    assert_eq!(f, correct);
}
