#[cfg(test)]
#[allow(unused_imports)]
use crate::monomial::Monomial;

#[test]
fn test_binary_op() {
    let deg1 = vec![1, 2, 3, 4, 5];
    let deg2 = vec![6, 7, 8, 9, 10];
    let deg12 = vec![7, 9, 11, 13, 15];

    let x1 = Monomial::from(deg1);
    let x2 = Monomial::from(deg2);

    let x12 = Monomial::from(deg12);

    assert_eq!(&x1 * &x2, x12);
    assert_eq!(x1.clone() * &x2, x12);
    assert_eq!(&x1 * x2.clone(), x12);
    assert_eq!(x1.clone() * x2.clone(), x12);
}
