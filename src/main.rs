#![allow(dead_code)]
mod linked1;
mod linked2;
mod linked3;
mod linked4;
mod linked5;

fn linked1_probes() {
    use linked1::*;
    size_l8();
    size_l8i();
    size_l2i();
    size_stupidthing();
    size_l64();
    // size_l65();
    size_a8();
    size_op8();
    size_ob8();
    size_oi64();
}

fn linked3_probes() {
    use linked3::*;
    test_cell();
}

fn main() {
    // linked1_probes();
    // linked3_probes();
    // linked4::List::new(&[3,6,8,9]);
    // profile_linked4_concat_huge();
}

/*
fn profile_linked4_concat_huge() {
    /*
    This requires:
    sudo apt install google-perftools libgoogle-perftools4 libgoogle-perftools-dev
    */
    use cpuprofiler::PROFILER;

    // Unlock the mutex and start the profiler
    PROFILER.lock().unwrap().start("./my-prof.profile").expect("Couldn't start");

    use linked4::List;
    // Testing performance here...
    let data_prev = vec![3,8,1,2,9,5,12,6,3,1,0,7,6,5,4,3,1,6,8,9,5,3,2,1,5,7,8,4,6];
    let mut data = data_prev.clone();
    for _ in 1..100 {
        data.extend(&data_prev);
    }
    let mut test = data.clone();
    let mut l = List::new(&data);
    for _ in 1..=100 {
        // Every time this is called, it has to find the tail. This is the most expensive operation.
        l.concat_copy(&List::new(&data));
        test.extend(&data);
    }
    let lvec = l.to_vec();
    assert_eq!(test, lvec);

    // Unwrap the mutex and stop the profiler
    PROFILER.lock().unwrap().stop().expect("Couldn't stop");

}*/
