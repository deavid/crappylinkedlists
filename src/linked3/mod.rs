#![allow(dead_code)]
use std::cell::Cell;
/*
Using `Cell<T>` to handle interior mutability of next
===========================================================================

So we get our Reference Linked Lists and tweak them so we can actually write.

Just working by references does not work even with a controller class that
holds the data. If the nodes were clonable, would this help? how?

We could get a copy of the leaf we're interested, modify it, and replace the
"next" we're interested in. But we still need mutable access to the previous
page to rewrite "next" or the whole linked list chain would need to be
rewritten.

There are two ways of making "next" mutable from an immutable struct:
Cell<T> and RefCell<T>.

RefCell works in runtime by counting the amount of borrows there are and
causing a panic if we try to do a mutable borrow while the read-only are
still there.

In contrast, Cell still works in compile time but it has serious limitations.
You can either overwrite it entirely or read a copy. If the data cannot be
copied (doesn't matter if it can be cloned) then the only way to read it is by
destructing the Cell itself in the process (by take() or replace()). You cannot
get read-only references to it.

For our purpose of rewriting next from an immutable struct this should work
because next is type Option<&Node>, and any immutable reference &x implements
the copy trait, and then, Option should also implement it.

Let's test it:
*/
#[derive(Debug)]
struct Num {
    v: i64,
}

pub fn test_cell() {
    let num = Num { v: 3 };
    let x: Cell<Option<&Num>> = Cell::new(None);
    let y: Cell<Option<&Num>> = Cell::new(Some(&num));
    println!("Cell get: x: {:#?}, y: {:#?}", x.get(), y.get());
}

/* Seems it's not a problem! So let's get to it: */
#[derive(Debug)]
pub struct LinkedList1<'a> {
    value: i64,
    next: Cell<Option<&'a LinkedList1<'a>>>,
}

pub struct IterLinkedList1<'a> {
    cursor: Option<&'a LinkedList1<'a>>,
}

/* Now I'll copy the implementation from linked2/LinkedList2 here: */

impl<'a> Iterator for IterLinkedList1<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.cursor.map(|c| c.value);
        /* I've replaced c.next with c.next(), just to avoid the extra .get() */
        self.cursor = self.cursor.map(|c| c.next()).flatten();
        ret
    }
}

impl<'a> LinkedList1<'a> {
    /* The constructor is quite simple: */
    pub fn new(value: i64, next: Option<&'a LinkedList1<'a>>) -> Self {
        LinkedList1 {
            value,
            next: Cell::new(next),
        }
    }

    /* Some getters and setters for public access: */
    pub fn value(&self) -> i64 {
        self.value
    }
    pub fn set_value(&mut self, value: i64) {
        self.value = value;
    }
    pub fn next(&self) -> Option<&Self> {
        /* This now will copy the option with the reference. Internally this
        will be just a nullable pointer being copied.*/
        self.next.get()
    }
    pub fn set_next(&self, next: Option<&'a LinkedList1<'a>>) -> Option<&LinkedList1<'a>> {
        /* Here we use replace instead to be able to write. Notice we no longer
        need a `&mut self`, an immutable reference is enough now. Also, we can
        return the old value easily, so why not? */
        self.next.replace(next)
    }
    pub fn iter(&'a self) -> IterLinkedList1 {
        /* I had to add the lifetime &'a to self to avoid confusion for Rust */
        IterLinkedList1 {
            cursor: Some(&self),
        }
    }
    fn tail(&self) -> &Self {
        let mut cur = self;
        /*while cur.next().is_some() {
            cur = cur.next().unwrap()
        }*/
        /* I've replaced the previous code with the following one. This avoids
        doing a copy of the pointer twice. */
        while let Some(next) = cur.next() {
            cur = next;
        }
        cur
    }

    /* Now there's no point of having a mutable tail function. If you want
    to mutate a page, replace it! */
    // fn tail_mut(&mut self) -> &mut Self { unimplemented!(); }

    fn insert(&self, item: &'a LinkedList1<'a>) {
        /* Instead of Option::replace we use Cell::replace, Some(x) is needed
        now to match the types  */
        let oldnext = self.next.replace(Some(item));
        let tail = item.tail();
        /* Now this simply works! */
        tail.next.replace(oldnext);
        /* Now the chain in item has been inserted in the middle. No data is
        left so we have nothing to return here. */
    }

    /* To change a page we should talk about replacing next instead. But there
    are two ways: replace and return the old chain, or replace and chain,
    returning the old item discarded. Anyway the signature is the same, because
    we would return always one item, in one case with next populated, and in the
    other next would always be None */
    fn replace(&self, item: &'a LinkedList1<'a>, chain: bool) -> Option<&'a LinkedList1<'a>> {
        unimplemented!();
    }
}