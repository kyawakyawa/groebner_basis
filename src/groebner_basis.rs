use std::cmp::Ordering;

use crate::polynomial::{Polynomial, PolynomialHandlers};
use crate::scalar::Integer;
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

        let lm_p = p.fetch_lm();

        match lm_p {
            Some(lm_p) => {
                for g in gs {
                    let lm_g = g.fetch_lm();
                    match lm_g {
                        Some(lm_g) => {
                            if lm_p.is_divisible_by(&lm_g) {
                                leaves[i] = false;
                                break;
                            }
                        }
                        None => {
                            panic!("found 0 polynomial");
                        }
                    }
                }
            }
            None => {
                panic!("found 0 polynomial");
            }
        }
    }

    let v = v
        .into_iter()
        .enumerate()
        .filter(|(i, _)| leaves[i.clone()])
        .map(|(_, g)| {
            assert_ne!(&g, &Polynomial::from((g.get_n(), g.get_monomial_order())));
            g.normalize()
        })
        .collect::<Vec<_>>();
    v
}

fn reduce_ideal_basis(v: Vec<Polynomial>) -> Vec<Polynomial> {
    let mut v = v;
    loop {
        let mut update_flag = false;

        for i in 0..(v.len()) {
            // gs = v - {v[i]}
            let gs = v
                .iter()
                .enumerate()
                .filter(|(j, _)| &i != j)
                .map(|(_, g)| {
                    assert_ne!(g, &Polynomial::from((g.get_n(), g.get_monomial_order()))); // Not zero polynomial
                    g
                })
                .collect::<Vec<_>>();

            if gs.is_empty() {
                continue;
            }

            let (_, r) = v[i].polynomial_divide_ref(&gs);

            assert_ne!(&r, &Polynomial::from((r.get_n(), r.get_monomial_order())));

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

fn to_reduced_groebner_basis(v: Vec<Polynomial>) -> Vec<Polynomial> {
    let v = to_minimal_groebner_basis(v);

    let v = reduce_ideal_basis(v);

    let mut v: Vec<Polynomial> = v.into_iter().map(|f| f.normalize()).collect();
    v.sort_by(|lhs, rhs| {
        let lm_l = lhs.fetch_lm();
        let lm_r = rhs.fetch_lm();

        match (lm_l, lm_r) {
            (Some(lm_l), Some(lm_r)) => {
                lm_r.cmp(&lm_l) // revert
            }
            (_, Some(_)) => {
                std::cmp::Ordering::Greater // revert
            }
            (Some(_), _) => {
                std::cmp::Ordering::Less // revert
            }
            (_, _) => std::cmp::Ordering::Equal,
        }
    }); // 出力は降順で
    v
}

fn detect_groebner_basis(v: &Vec<Polynomial>) -> bool {
    detect_groebner_basis_ref(&v.iter().map(|f| f).collect())
}

fn detect_groebner_basis_ref(v: &Vec<&Polynomial>) -> bool {
    if v.is_empty() {
        return false;
    }

    let monomial_order = v[0].get_monomial_order();
    let n = v[0].get_n();

    let zero_polynomial = Polynomial::from((n, monomial_order));

    v.iter().enumerate().all(|(i, fi)| {
        v.iter()
            .enumerate()
            .filter(|(j, _)| i != *j)
            .all(|(_, fj)| {
                let s_poly = polynomial::s_polynomial(fi, fj);
                match s_poly {
                    Some(s_poly) => &s_poly.polynomial_mod_ref(v) == &zero_polynomial,
                    None => {
                        panic!(
                            "faild to compute s polynomial\n fi -> {:?}\n fj -> {:?},",
                            fi, fj
                        );
                    }
                }
            })
    })
}

#[derive(Debug, Clone)]
struct PolynomialPair {
    lm_fi: Monomial,
    lm_fj: Monomial,

    s_polynomial_suger: Integer,

    lcm: Monomial,

    ij: (usize, usize),
}

impl From<(&Vec<Polynomial>, &Vec<Integer>, (usize, usize))> for PolynomialPair {
    fn from((fs, total_degrees, ij_): (&Vec<Polynomial>, &Vec<Integer>, (usize, usize))) -> Self {
        assert_ne!(ij_.0, ij_.1);

        let ij_ = if ij_.0 < ij_.1 { ij_ } else { (ij_.1, ij_.0) };

        let i = ij_.0;
        let j = ij_.1;

        let fi = &fs[i];
        let fj = &fs[j];

        let total_degree_i = &total_degrees[i];
        let total_degree_j = &total_degrees[j];

        assert_eq!(fi.get_monomial_order(), fj.get_monomial_order());
        assert_eq!(fi.get_n(), fj.get_n());

        let lm_fi_ = fi.fetch_lm();
        let lm_fj_ = fj.fetch_lm();

        match (lm_fi_, lm_fj_) {
            (Some(lm_fi_), Some(lm_fj_)) => {
                let lcm_ = monomial::lcm(&lm_fi_, &lm_fj_);

                assert!(lcm_.is_divisible_by(&lm_fi_));
                assert!(lcm_.is_divisible_by(&lm_fj_));

                let ri = &lcm_ / &lm_fi_;
                let rj = &lcm_ / &lm_fj_;

                let s_polynomial_suger_ = std::cmp::max(
                    ri.fetch_total_degree() + total_degree_i,
                    rj.fetch_total_degree() + total_degree_j,
                );

                Self {
                    lm_fi: lm_fi_,
                    lm_fj: lm_fj_,

                    s_polynomial_suger: s_polynomial_suger_,

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

impl PartialEq for PolynomialPair {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(&other) == Ordering::Equal
    }
}

impl Eq for PolynomialPair {}

impl PartialOrd for PolynomialPair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PolynomialPair {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.ij == other.ij {
            return Ordering::Equal;
        }

        let ord_suger = self.s_polynomial_suger.cmp(&other.s_polynomial_suger);

        if &ord_suger != &Ordering::Equal {
            return ord_suger;
        }

        let ord_lcm = self.lcm.cmp(&other.lcm);
        if &ord_lcm != &Ordering::Equal {
            return ord_lcm;
        }

        self.ij.cmp(&other.ij)
    }
}

pub fn compute_groebner_basis(fs: Vec<Polynomial>) -> Vec<Polynomial> {
    if fs.is_empty() {
        return Vec::new();
    }
    let monomial_order = fs[0].get_monomial_order();
    let n = fs[0].get_n();

    let zero_polynomial = Polynomial::from((n, monomial_order));

    let fs = reduce_ideal_basis(fs);

    let fs = fs
        .into_iter()
        .filter(|f| f != &zero_polynomial)
        .collect::<Vec<Polynomial>>();

    let mut fs = fs;
    fs.sort_by(|lhs, rhs| {
        let ord_total_degree = lhs.fetch_total_degree().cmp(&rhs.fetch_total_degree()); // TODO 高速化
        if ord_total_degree != Ordering::Equal {
            return ord_total_degree;
        }
        let lm_l = lhs.fetch_lm();
        let lm_r = rhs.fetch_lm();

        match (lm_l, lm_r) {
            (Some(lm_l), Some(lm_r)) => lm_l.cmp(&lm_r),
            (_, Some(_)) => std::cmp::Ordering::Less,
            (Some(_), _) => std::cmp::Ordering::Greater,
            (_, _) => std::cmp::Ordering::Equal,
        }
    });

    let mut t = fs.len();

    let mut total_degrees = fs
        .iter()
        .map(|f| {
            let total_degree = f.fetch_total_degree();
            match total_degree {
                Some(total_degree) => total_degree,
                None => {
                    panic!("found 0 polynomial");
                }
            }
        })
        .collect::<Vec<Integer>>();

    let mut pairs = fs
        .iter()
        .enumerate()
        .map(|(i, _)| {
            fs.iter()
                .enumerate()
                .filter(|(j, _)| &i < j)
                .map(|(j, _)| PolynomialPair::from((&fs, &total_degrees, (i.clone(), j.clone()))))
                .collect::<Vec<PolynomialPair>>()
        })
        .flatten()
        .collect::<BTreeSet<PolynomialPair>>();

    assert_eq!(pairs.len(), t * (t - 1) / 2);

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
                            }
                        })
                        .any(|(k, _)| {
                            let s_ik =
                                PolynomialPair::from((&fs, &total_degrees, (i.clone(), k.clone())));
                            let s_jk =
                                PolynomialPair::from((&fs, &total_degrees, (j.clone(), k.clone())));
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
                let s = s.polynomial_mod(&fs);
                if s != zero_polynomial {
                    let ft = s;
                    let total_degree = ft.fetch_total_degree();

                    match total_degree {
                        Some(total_degree) => {
                            fs.push(ft);
                            total_degrees.push(total_degree);

                            for i in 0..t {
                                pairs.insert(PolynomialPair::from((&fs, &total_degrees, (i, t))));
                            }

                            t = t + 1;
                        }
                        None => {
                            panic!("0 polynomial found");
                        }
                    }
                    if pairs.len() > 10000 && detect_groebner_basis(&fs) {
                        break;
                    }
                }
            }
        }
        pairs.remove(&polynomial_pair);
    }

    to_reduced_groebner_basis(fs)
}

mod test;
