#![allow(dead_code)]
/* 
Stack-pointer Linked Lists
===========================================================================

What if everything has to have a lifetime?

Regular value references in Rust, expressed by &variable, are proved correct
by the compiler at compile time. This usually means that these variables are
created and destructed on the stack. They can still be used in the heap as long
as you create them by using a Box or a Rc object. But for the majority of the
code, they will be treated as if they were going to be destructed on exiting
a function or piece of code, which is going to give us real headaches on the
lifetimes and borrow checker.

It's going to be painful, but it's elightening.

Let's start with a simple structure. This fails because we need to specify a
lifetime:

struct LinkedList1 {
    value: i64,
    next: Option<&LinkedList1>,
    //           ^ expected named lifetime parameter    
}

There are two ways for fixing this, one is to have a 'static lifetime and the
other is to use a generic approach defining a lifetime on the struct itself.

Let's try the 'static approach:
*/
pub struct LinkedList1 {
    value: i64,
    next: Option<&'static LinkedList1>,
}

/*
This works, but it has the problem that the reference has to be static, this
means that the object this pointer refers to is valid for the whole program. 
So it's never cleaned up.

See what happens:
*/
fn test_ll1() {
    // let n1 = LinkedList1{
    //     value: 0,
    //     next: None,
    // };
    // let n2 = LinkedList1{
    //     value: 0,
    //     next: Some(&n1),
    //     ^^^
    //     |
    //     borrowed value does not live long enough
    //     requires that `n1` is borrowed for `'static`        
    // };

    static N1: LinkedList1 = LinkedList1 {
        value: 0,
        next: None,
    };
    let n2 = LinkedList1 {
        value: 0,
        next: Some(&N1),
    };
    println!("{}", n2.value);
    let n1_value = match n2.next {
        Some(x) => x.value,
        None => unreachable!(),
    };
    println!("{}", n1_value);
}

/*
This is a bit useless because N1 value is effectively being built inside the 
binary and is into memory from the start to the end of the program. For some
values, sometimes, this does make sense. But for LinkedLists this is absurd.

So we have to go with the generics approach to define a lifetime:
*/

pub struct LinkedList2<'a> {
    value: i64,
    next: Option<&'a LinkedList2<'a>>
}

/*
If you look closely you'll detect that we have to specify the lifetime twice
on the Option. First, we define the lifetime of the borrow itself as &'a, then
we also define the lifetime inside LinkedList2 as <'a>. What does this mean?

It means that the parent and children shoult have the same lifetime, or to be
more precise, the children lifetime must outlive the parent. 

This makes sense. The parent cannot be freed if it has children, so the parent
must outlive them.

The size of this struct is going to be two words (16bytes). The first word is
the value, and the next word is the pointer to memory. 

Let's begin with an implementation for this:
*/
impl<'a> LinkedList2<'a> {

    /* The constructor is quite simple: */
    pub fn new(value: i64, next: Option<&'a LinkedList2<'a>>) -> Self {
        LinkedList2 {
            value,
            next,
        }
    }
}