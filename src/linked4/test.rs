use super::*;

#[test]
fn test_create() {
    let data = vec![3,8,1,2];
    let l = List::new(&data);
    let lvec = l.to_vec();
    assert_eq!(data, lvec);
}

#[test]
fn test_concat() {
    let data = vec![3,8,1,2];
    let mut test = data.clone();
    let mut l = List::new(&data);
    for _ in 1..=10 {
        l.concat_copy(&List::new(&data));
        test.extend(&data);
    }
    let lvec = l.to_vec();
    assert_eq!(test, lvec);
}

#[test]
fn test_concat_big() {
    // This one does stack overflow if drop trait is not implemented
    let data = vec![3,8,1,2,9,5,12,6,3,1,0,7,6,5,4,3,1,6,8,9,5,3,2,1,5,7,8,4,6];
    let mut test = data.clone();
    let mut l = List::new(&data);
    for _ in 1..=1000 {
        l.concat_copy(&List::new(&data));
        test.extend(&data);
    }
    let lvec = l.to_vec();
    assert_eq!(test, lvec);
}

#[test]
fn test_concat_huge() {
    // Testing performance here...
    let data_prev = vec![3,8,1,2,9,5,12,6,3,1,0,7,6,5,4,3,1,6,8,9,5,3,2,1,5,7,8,4,6];
    let mut data = data_prev.clone();
    for _ in 1..100 {
        data.extend(&data_prev);
    }
    let mut test = data.clone();
    let mut l = List::new(&data);
    for _ in 1..=100 {
        // Concat copy has to do a tail, so each time tries to find the last item. This is expensive.
        l.concat_copy(&List::new(&data));
        test.extend(&data);
    }
    let lvec = l.to_vec();
    assert_eq!(test, lvec);
}