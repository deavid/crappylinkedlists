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
    let mut test = Box::new(data.clone());
    let mut l = List::from_vec(&data);
    for _ in 1..=100 {
        l.concat(List::from_vec(&data));
        test.extend(data.iter());
    }
    println!("test1");
    let lvec = Box::new(l.to_vec());
    println!("test2");
    assert_eq!(test, lvec);
    println!("test3");
}

#[test]
fn test_pop_first() {
    let v = vec![3, 4, 0, 1, 2, 5, 6, 7, 8];
    let empty: Vec<i64> = Vec::new();
    let mut l = List::from_vec(&v);
    let mut got: Vec<i64> = Vec::new();
    while let Some(val) = l.pop_first() {
        got.push(val);
    }
    assert_eq!(v, got);
    assert_eq!(empty, l.to_vec());
    assert_eq!(empty, l.to_vec_rev());
}

#[test]
fn test_pop_last() {
    let v = vec![3, 4, 0, 1, 2, 5, 6, 7, 8];
    let empty: Vec<i64> = Vec::new();
    let want: Vec<i64> = v.iter().rev().cloned().collect();
    let mut l = List::from_vec(&v);
    let mut got: Vec<i64> = Vec::new();
    while let Some(val) = l.pop_tail() {
        got.push(val);
    }
    assert_eq!(want, got);
    assert_eq!(empty, l.to_vec());
    assert_eq!(empty, l.to_vec_rev());
}

#[test]
fn test_insert_first() {
    let v = vec![3, 4, 0, 1, 2, 5, 6, 7, 8];
    let fv = vec![9, 11, 15 ,32];
    let mut l = List::from_vec(&v);
    for elem in fv.iter().rev() {
        l.insert_first(*elem);
    }
    let got: Vec<i64> = l.to_vec();
    let want: Vec<i64> = fv.iter().cloned().chain(v).collect();
    assert_eq!(want, got);
    
    let got: Vec<i64> = l.to_vec_rev();
    let want: Vec<i64> = want.iter().rev().cloned().collect();
    assert_eq!(want, got);
}
