use crate::polynomial::{Polynomial, PolynomialHandlers};

fn to_minimal_groebner_basis(v: Vec<Polynomial>) -> Vec<Polynomial> {
    // TODO Linked List がまともになったらLinked Listを使う
    let mut leaves = Vec::new();
    leaves.resize(v.len(), true);

    for (i, p) in v.iter().enumerate() {
        let g = v
            .iter()
            .enumerate()
            .filter(|(j, _)| &i != j && leaves[j.clone()])
            .map(|(_, g)| g)
            .collect::<Vec<_>>();

        let (_, r) = p.polynomial_divide_ref(&g);

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

pub fn to_reduced_groebner_basis(v: Vec<Polynomial>) -> Vec<Polynomial> {
    let v = to_minimal_groebner_basis(v);

    Vec::new()
}
