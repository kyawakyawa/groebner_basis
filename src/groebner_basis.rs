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

        if gs.is_empty() {
            continue;
        }

        let (_, r) = p.polynomial_divide_ref(&gs);

        let zero = Polynomial::from((r.get_n(), r.get_monomial_order()));
        if r == zero {
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

            if gs.is_empty() {
                continue;
            }

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

    let mut v: Vec<Polynomial> = v.into_iter().map(|f| f.normalize()).collect();
    v.sort_by(|lhs, rhs| rhs.cmp(lhs)); // 出力は降順で
    v
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct PolynomialPair {
    lm_fi: Monomial,
    lm_fj: Monomial,

    lcm: Monomial,

    ij: (usize, usize),
}

impl From<(&Vec<Polynomial>, (usize, usize))> for PolynomialPair {
    fn from((fs, ij_): (&Vec<Polynomial>, (usize, usize))) -> Self {
        assert_ne!(ij_.0, ij_.1);

        let ij_ = if ij_.0 < ij_.1 { ij_ } else { (ij_.1, ij_.0) };

        let i = ij_.0;
        let j = ij_.1;

        let fi = &fs[i];
        let fj = &fs[j];

        assert_eq!(fi.get_monomial_order(), fj.get_monomial_order());
        assert_eq!(fi.get_n(), fj.get_n());

        let lm_fi_ = fi.fetch_lm();
        let lm_fj_ = fj.fetch_lm();

        match (lm_fi_, lm_fj_) {
            (Some(lm_fi_), Some(lm_fj_)) => {
                let lcm_ = monomial::lcm(&lm_fi_, &lm_fj_);
                Self {
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

impl PartialOrd for PolynomialPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PolynomialPair {
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
    fs.sort();
    let mut t = fs.len();

    let mut pairs = fs
        .iter()
        .enumerate()
        .map(|(i, _)| {
            fs.iter()
                .enumerate()
                .filter(|(j, _)| &i < j)
                .map(|(j, _)| PolynomialPair::from((&fs, (i.clone(), j.clone()))))
                .collect::<BTreeSet<PolynomialPair>>()
        })
        .flatten()
        .collect::<BTreeSet<PolynomialPair>>();

    loop {
        // TODO: pop_firstを使う
        let polynomial_pair = pairs.iter().next();
        let (polynomial_pair, can_ignore) = match polynomial_pair {
            Some(polynomial_pair) => {
                let (i, j) = &polynomial_pair.ij;

                let condition0 =
                    polynomial_pair.lcm == &polynomial_pair.lm_fi * &polynomial_pair.lm_fj;
                if condition0 {
                    (polynomial_pair.clone(), true)
                } else {
                    let can_ignore = fs
                        .iter()
                        .enumerate()
                        .filter(|(k, _)| i != k && j != k)
                        .filter(|(_, fk)| match fk.fetch_lm() {
                            Some(lk_lm) => polynomial_pair.lcm.is_divisible_by(&lk_lm),
                            None => {
                                panic!("found 0 polynomial");
                            },
                        })
                        .any(|(k, _)| {
                            let s_ik = PolynomialPair::from((&fs, (i.clone(), k.clone())));
                            let s_jk = PolynomialPair::from((&fs, (j.clone(), k.clone())));
                            match (pairs.get(&s_ik), pairs.get(&s_jk)) {
                                (None, None) => true,
                                (_, _) => false,
                            }
                        });
                    (polynomial_pair.clone(), can_ignore)
                }
            }
            None => {
                break;
            }
        };

        if !can_ignore {
            let fi = &fs[polynomial_pair.ij.0];
            let fj = &fs[polynomial_pair.ij.1];

            let s = polynomial::s_polynomial(fi, fj);

            if let Some(s) = s {
                let (_, s) = s.polynomial_divide(&fs);
                if s != zero_polynomial {
                    let ft = s;
                    fs.push(ft);

                    for i in 0..t {
                        pairs.insert(PolynomialPair::from((&fs, (i, t))));
                    }

                    t = t + 1;
                }
            }
        }
        pairs.remove(&polynomial_pair);
    }

    to_reduced_groebner_basis(fs)
}

mod test;
