use super::*;

#[test]
fn test_create() {
    let want = vec![3, 4, 0, 1, 2, 5, 6, 7, 8, 9];
    let l = List::from_vec(&want);
    let got = l.to_vec();
    assert_eq!(want, got);
}

#[test]
fn test_rev_iter() {
    let v = vec![3, 4, 0, 1, 2, 5, 6, 7, 8];
    let l = List::from_vec(&v);
    let got: Vec<i64> = l.to_vec_rev();
    let want: Vec<i64> = v.iter().rev().cloned().collect();
    assert_eq!(want, got);
}

#[test]
fn test_concat() {
    let data = vec![3, 8, 1, 2];
    let mut test = data.clone();
    let mut l = List::from_vec(&data);
    for _ in 1..=10 {
        l.concat(List::from_vec(&data));
        test.extend(&data);
    }
    let lvec = l.to_vec();
    assert_eq!(test, lvec);
}

#[test]
fn test_concat_huge() {
    // Testing performance here...
    let data_prev = vec![
        3, 8, 1, 2, 9, 5, 12, 6, 3, 1, 0, 7, 6, 5, 4, 3, 1, 6, 8, 9, 5, 3, 2, 1, 5, 7, 8, 4, 6,
    ];
    let mut data = data_prev.clone();
    for _ in 1..10 {
        data.extend(&data_prev);
    }
    let mut test = data.clone();
    let mut l = List::from_vec(&data);
    for _ in 1..=1000 {
        l.concat(List::from_vec(&data));
        test.extend(&data);
    }
    let lvec = l.to_vec();
    assert_eq!(test, lvec);
}
