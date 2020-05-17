#[macro_use]
extern crate bencher;
use crappylinkedlists::linked5::List;
use std::mem::size_of;
use bencher::Bencher;

fn create_new(bench: &mut Bencher) {
    bench.iter(|| {
        List::new()
    })
}

fn create_from_vec_10(bench: &mut Bencher) {
    let d: Vec<i64> = vec![1,2,3,4,5,6,7,8,9,10];
    bench.iter(|| {
        List::from_vec(&d)
    });
    bench.bytes = (d.len() * size_of::<i64>()) as u64;
    
}

fn create_from_vec_1k(bench: &mut Bencher) {
    let data_prev: Vec<i64> = vec![1,2,3,4,5,6,7,8,9,10];
    let mut d = data_prev.clone();
    for _ in 1..100 {
        d.extend(&data_prev);
    }

    bench.iter(|| {
        List::from_vec(&d)
    });
    bench.bytes = (d.len() * size_of::<i64>()) as u64;
}


fn create_from_concat_10x100(bench: &mut Bencher) {
    let d: Vec<i64> = vec![1,2,3,4,5,6,7,8,9,10];
    let n = 100;
    bench.iter(|| {
        let mut l = List::from_vec(&d);
        for _ in 0..n {
            l.concat(List::from_vec(&d))
        }
        l
    });
    bench.bytes = (n * d.len() * size_of::<i64>()) as u64;
    
}


benchmark_group!(benches, 
    create_new, 
    create_from_vec_10, 
    create_from_vec_1k,
    create_from_concat_10x100,
);
benchmark_main!(benches);