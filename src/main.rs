mod linked1;

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

fn main() {
    linked1_probes();
}
