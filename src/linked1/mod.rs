#![allow(dead_code)]
use std::mem::size_of;
/* 
Value-Only Linked Lists
===========================================================================

What happens if we refuse to use pointers at all costs?

Let's say we don't want pointers. Just regular values. This implies that the
next value is not a pointer but the actual value.

The following approach does not work. Values inside an struct are "inlined"
in memory, so in order to reserve memory for this struct you'll actually 
need to store the child inside. And because they can be as long as you want,
that means that you need to reserve "infinite" memory just in case to 
guarantee that it fits.
*/
struct LinkedList1 {
    value: i64,
    // next: Option<LinkedList1>, // <-- recursive type has infinite size
    /*     help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point 
                 to make `linked1::LinkedList` representable*/
}

/*
The way to make this work would be to store a pointer to memory instead of using
the full contents inlined. But rust has many ways to do this.

But.

It fails because it has infinite size. Is it possible to declare something
that inlines the result without going infinite? i.e. It has finite size.
*/

struct LinkedList2<T> {
    value: i64,
    next: Option<T>,
}

/* For convenience we call the list "L" */
type L<T> = LinkedList2<T>;

/* 
And now we can abuse generics to create buckets of list sizes that can be 
chained as long as we want:
*/

type L4T<T> = L<L<L<L<T>>>>;
type L8T<T> = L4T<L4T<T>>;

/* And now we could have a list with a maximum of 8 items: */
type L8 = L8T<()>;

pub fn size_l8() {
    println!("Size of L8: {}", size_of::<L8>()) // 72 bytes
}

/*
This always stores 72 bytes which is the equivalent of 8 bytes for each i64
and an extra of 8 bytes for... the last empty option maybe?

If instead of using a tail, we just store the value on the last "next", we
should be able of getting rid of the last 8 bytes:
*/

type L8i = L<L<L<L4T<i64>>>>;

pub fn size_l8i() {
    println!("Size of L8i: {}", size_of::<L8i>()) // 72 bytes
}

/* It's still 72 bytes long. Let's try with 2 items instead: */
type L2i = L<i64>;

pub fn size_l2i() {
    println!("Size of Li2: {}", size_of::<L2i>()) // 24 bytes
}

/* Where are the extra 8 bytes coming from? Let's build a stupid thing: */

struct StupidThing {
    v: i64,
    n: i64,
}

pub fn size_stupidthing() {
    println!("Size of StupidThing: {}", size_of::<StupidThing>()) // 16 bytes
}

/* 
So we agree that {i64, i64} is 16 bytes, but {i64, Option<i64>} is 24 bytes.
But {i64, Option<{i64, Option<i64>>} does not add another 8 bytes of overhead.

Why does this happen? Well, Option has to take some storage, a bit in order
to store if it has data or not. But because there's memory aligment and
probably we're all compiling these things in 64bit platforms, you can stack
up to 8 bytes in a single word. Anything bellow that would need padding. 

Rust is smart and does pack all those bytes together so all the data is aligned.

You want proof? Let's build a 64 item list:
*/
type L32T<T> = L8T<L8T<L8T<L8T<T>>>>;
type L64 = L32T<L32T<()>>;
pub fn size_l64() {
    println!("Size of L64: {}", size_of::<L64>()) // 520 bytes
}

/*
Now it's taking 64*8 bytes (512B) for values plus 8 bytes for options.

8 bytes contain 64 bits, each one can be used to hold one Option to be Some or
None. That's pretty efficient.

So what happens if we build one with 65?
Currently Rust fails to build this because there is too much recursivity
ongoing when expanding the template. The following code does not work:
*/
/*
type L65 = L<L64>;
pub fn size_l65() {
    println!("Size of L64: {}", size_of::<L65>()) // 128 bytes
}
*/

/*
You might say, this is stupid, it's simply an array [Option<i64>; 8] or 
whatever. And you will be right. Or is it? Let's check!
*/

type A8 = [Option<i64>; 8];
pub fn size_a8() {
    println!("Size of A8: {}", size_of::<A8>()) // 128 bytes
}

/* 
It's 128 bytes, not 72 bytes! Rust cannot pack the options together for an
array, so it's using two words per item.
*/

/*
And yes, the examples on this file do not follow the definition of what
a Linked List is: The items are not linked together, because we're not storing
links, we re inlining them! They resemble to linked lists, sometimes the access
methods may resemble as well, but they're technically not a Linked List.

This served us to learn more about generics and the templating system in Rust.
*/

/*
For pointers, this is another story. Because they're non nullable, Rust can
effectively do an Option<&T> or Option<Box> into a single usize:
*/

type OP8 = Option<&'static L8>;
pub fn size_op8() {
    println!("Size of OP8: {}", size_of::<OP8>()) // 8 bytes
}
type OB8 = Option<Box<L8>>;
pub fn size_ob8() {
    println!("Size of OB8: {}", size_of::<OB8>()) // 8 bytes
}
type OI64 = Option<i64>;
pub fn size_oi64() {
    println!("Size of OI64: {}", size_of::<OI64>()) // 16 bytes
}

/*
As we can see, storing pointers both in stack form or heap form makes the option
to be effectively of zero size, because it uses the NULL (0x00) value to store
None. For regular values, the Option takes 1 word (8 bytes) plus the type size.
*/

/*
That's all for stack-only values! Let's experiment with stack pointers!
*/