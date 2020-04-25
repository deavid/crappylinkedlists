#![allow(dead_code)]
mod linked1;
mod linked2;
mod linked3;
mod linked4;

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
    linked4::List::new(&[3,6,8,9]);

}
