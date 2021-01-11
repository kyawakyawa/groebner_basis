use std::cmp::Ordering;

use crate::polynomial::{Polynomial, PolynomialHandlers};
use crate::{monomial, polynomial};
use monomial::{Monomial, MonomialHandlers};

use std::collections::BTreeSet;

fn to_minimal_groebner_basis(v: Vec<Polynomial>) -> Vec<Polynomial> {
    // TODO Linked List がまともになったらLinked Listを使う
    let mut leaves = Vec::new();
    leaves.resize(v.len(), true);

    for (i, p) in v.iter().enumerate() {
        let gs = v
            .iter()
            .enumerate()
            .filter(|(j, _)| &i != j && leaves[j.clone()])
            .map(|(_, g)| g)
            .collect::<Vec<_>>();

        let (_, r) = p.polynomial_divide_ref(&gs);

        if r == Polynomial::from(r.get_n()) {
            leaves[i] = false;
        }
    }

    let v = v
        .into_iter()
        .enumerate()
        .filter(|(i, _)| leaves[i.clone()])
        .map(|(_, g)| g.normalize())
        .collect::<Vec<_>>();
    v
}

fn to_reduced_groebner_basis(v: Vec<Polynomial>) -> Vec<Polynomial> {
    let mut v = to_minimal_groebner_basis(v);

    loop {
        let mut update_flag = false;

        for i in 0..(v.len()) {
            let gs = v
                .iter()
                .enumerate()
                .filter(|(j, _)| &i != j)
                .map(|(_, g)| g)
                .collect::<Vec<_>>();

            let (_, r) = v[i].polynomial_divide_ref(&gs);

            if v[i] != r {
                update_flag = true;
                v[i] = r;
            }
        }

        if !update_flag {
            break;
        }
    }

    v
}

struct PolynomialPair<'a> {
    fi: &'a Polynomial,
    fj: &'a Polynomial,

    lm_fi: Monomial,
    lm_fj: Monomial,

    lcm: Monomial,

    ij: (usize, usize),
}

impl<'a> From<(&'a Polynomial, &'a Polynomial, (usize, usize))> for PolynomialPair<'a> {
    fn from((fi_, fj_, ij_): (&'a Polynomial, &'a Polynomial, (usize, usize))) -> Self {
        assert_ne!(ij_.0, ij_.1);

        let (fi_, fj_, ij_) = if ij_.0 < ij_.1 {
            (fi_, fj_, ij_)
        } else {
            (fj_, fi_, (ij_.1, ij_.0))
        };

        assert_eq!(fi_.get_monomial_order(), fj_.get_monomial_order());
        assert_eq!(fi_.get_n(), fj_.get_n());

        let lm_fi_ = fi_.lm();
        let lm_fj_ = fj_.lm();

        match (lm_fi_, lm_fj_) {
            (Some(lm_fi_), Some(lm_fj_)) => {
                let lcm_ = monomial::lcm(&lm_fi_, &lm_fj_);
                Self {
                    fi: fi_,
                    fj: fj_,

                    lm_fi: lm_fi_,
                    lm_fj: lm_fj_,

                    lcm: lcm_,

                    ij: ij_,
                }
            }
            (_, _) => {
                panic!("zero polynomial");
                // Monomial::from((fi_.get_n(), fi_.get_monomial_order()))
            }
        }
    }
}

impl<'a> PartialEq for PolynomialPair<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.ij == other.ij
    }
}

impl<'a> Eq for PolynomialPair<'a> {}

impl<'a> PartialOrd for PolynomialPair<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for PolynomialPair<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.lcm.cmp(&other.lcm)
    }
}
pub fn compute_groebner_basis(fs: Vec<Polynomial>) -> Vec<Polynomial> {
    if fs.is_empty() {
        return Vec::new();
    }
    let monomial_order = fs[0].get_monomial_order();
    let n = fs[0].get_n();

    let zero_polynomial = Polynomial::from((n, monomial_order));

    let fs = fs
        .into_iter()
        .filter(|f| f != &zero_polynomial)
        .collect::<Vec<Polynomial>>();

    let mut fs = fs;
    let mut t = fs.len();

    let mut B = fs
        .iter()
        .enumerate()
        .map(|(i, gi)| {
            fs.iter()
                .enumerate()
                .filter(|(j, _)| &i < j)
                .map(|(j, gj)| PolynomialPair::from((gi, gj, (i.clone(), j.clone()))))
                .collect::<BTreeSet<PolynomialPair>>()
        })
        .flatten()
        .collect::<BTreeSet<PolynomialPair>>();

    loop {
        // TODO: pop_firstを使う
        let mut polynomial_pair = B.iter().next();
        match polynomial_pair {
            Some(polynomial_pair) => {
                let fi = polynomial_pair.fi;
                let fj = polynomial_pair.fj;

                let (i, j) = &polynomial_pair.ij;

                let condition0 =
                    polynomial_pair.lcm == &polynomial_pair.lm_fi * &polynomial_pair.lm_fj;

                let condition1 = fs
                    .iter()
                    .enumerate()
                    .filter(|(k, hoge)| i != k && j != k)
                    .filter(|(k, fk)| {
                        if i == k || j == k {
                            return false;
                        }

                        match fk.lm() {
                            Some(lk_lm) => polynomial_pair.lcm.is_divisible_by(&lk_lm),
                            None => {
                                true // 零多項式も要らない
                            }
                        }
                    })
                    .any(|(k, fk)| {
                        let s_ik = PolynomialPair::from((fi, fk, (i.clone(), k.clone())));
                        let s_jk = PolynomialPair::from((fj, fk, (j.clone(), k.clone())));
                        match (B.get(&s_ik), B.get(&s_jk)) {
                            (None, None) => true,
                            (_, _) => false,
                        }
                    });
                let can_ignore = condition0 && condition1;
                if !can_ignore {
                    let s = polynomial::s_polynomial(fi, fj);
                    if let Some(s) = s {
                        let (_, s) = s.polynomial_divide(&fs);
                        if s != zero_polynomial {
                            t = t + 1;
                            let ft = s;
                        }
                    }
                }
            }
            None => {
                break;
            }
        }
    }

    fs
}
