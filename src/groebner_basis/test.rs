#[cfg(test)]
use crate::groebner_basis;

#[allow(unused_imports)]
use crate::monomial;
#[allow(unused_imports)]
use crate::monomial::{Monomial, MonomialHandlers};
#[allow(unused_imports)]
use crate::polynomial;
#[allow(unused_imports)]
use crate::polynomial::{Polynomial, PolynomialHandlers};
#[allow(unused_imports)]
use crate::scalar::{Integer, Rational};

#[allow(dead_code)]
fn check_reduced_groebner_basis_property(fs: &Vec<Polynomial>, gs: &Vec<Polynomial>) {
    if gs.is_empty() {
        panic!("G is empty");
    }
    for g in gs {
        let lc = g.fetch_lc();
        if let Some(lc) = lc {
            assert_eq!(lc, Rational::from(1));
        } else {
            panic!("zero polynomial");
        }
    }

    let monomial_order = gs[0].get_monomial_order();
    let n = gs[0].get_n();

    let zero_polynomial = Polynomial::from((n, monomial_order));

    for i in 0..gs.len() {
        let gs_ = gs
            .iter()
            .enumerate()
            .filter(|(j, _)| &i != j)
            .map(|(_, f)| f)
            .collect::<Vec<_>>();
        let (_, r) = gs[i].polynomial_divide_ref(&gs_);
        assert_ne!(r, zero_polynomial);
    }

    for f in fs {
        let (_, r) = f.polynomial_divide(gs);
        assert_eq!(r, zero_polynomial);
    }
}

#[test]
fn test_groebner_basis() {
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
    let fs_copy = fs.clone();
    println!("compute groebner basis");
    for f in fs.iter() {
        println!("{}", f);
    }

    let gb = groebner_basis::compute_groebner_basis(fs);

    println!("fin");
    for g in gb.iter() {
        println!("{}", g);
    }

    check_reduced_groebner_basis_property(&fs_copy, &gb);

    let mut correct_g0 = Polynomial::from((3, monomial::MonomialOrder::Lex));
    correct_g0.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(1), Integer::from(0), Integer::from(0)]),
    );
    correct_g0.add_term(
        Rational::from(-1),
        Monomial::from(vec![Integer::from(0), Integer::from(0), Integer::from(1)]),
    );

    let mut correct_g1 = Polynomial::from((3, monomial::MonomialOrder::Lex));
    correct_g1.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(0), Integer::from(1), Integer::from(0)]),
    );
    correct_g1.add_term(
        Rational::from(-2),
        Monomial::from(vec![Integer::from(0), Integer::from(0), Integer::from(2)]),
    );

    let mut correct_g2 = Polynomial::from((3, monomial::MonomialOrder::Lex));
    correct_g2.add_term(
        Rational::from(1),
        Monomial::from(vec![Integer::from(0), Integer::from(0), Integer::from(4)]),
    );
    correct_g2.add_term(
        Rational::from(1) / Rational::from(2),
        Monomial::from(vec![Integer::from(0), Integer::from(0), Integer::from(2)]),
    );
    correct_g2.add_term(
        Rational::from(-1) / Rational::from(4),
        Monomial::from(vec![Integer::from(0), Integer::from(0), Integer::from(0)]),
    );

    assert_eq!(gb[0], correct_g0);
    assert_eq!(gb[1], correct_g1);
    assert_eq!(gb[2], correct_g2);
}
