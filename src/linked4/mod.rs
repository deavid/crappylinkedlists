#![allow(dead_code)]
/*
Adding Box to the mix
===========================================================================

And we should finally get something that hopefully works.

We will use Box<T> for "next". Why Box? Because we want full ownership on 
the child, so we actually hold memory and we're responsible for freeing it.

We cannot use Cell for next. Box<T> is not a Copy type. Box can implement 
clone if the type implements cloning. But implementing cloning would mean to
recursively copy all its contents. This is terribly inefficient.

The actual solution would be to use RefCell instead. But this is a runtime
checked solution which implies counting how many borrows are there. I want
to avoid this if possible, so let's go without Cell for now.
*/

#[derive(Debug)]
struct LinkedList1 {
    value: i64,
    next: Option<Box<LinkedList1>>,
}


pub struct IterLinkedList1<'a> {
    /* Notice this one is still a reference. Why? Iterators are expected to be
    consumed. It doesn't make much sense to leave an iterator floating around
    permanently. */
    cursor: Option<&'a LinkedList1>,
}

impl<'a> Iterator for IterLinkedList1<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.cursor.map(|c| c.value);
        /* Now we have to use Option::as_deref() so it swaps the Box with 
        a reference */
        self.cursor = match self.cursor {
            Some(node) => node.next.as_deref(),
            None => None,
        };
        ret
    }
}


impl LinkedList1 {
    /* This new function is now a bit pointless. But I'll keep it. */
    pub fn new(value: i64, next: Option<Box<LinkedList1>>) -> Self {
        LinkedList1 {
            value,
            next,
        }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
    pub fn set_value(&mut self, value: i64) {
        self.value = value;
    }
    pub fn next(&self) -> Option<&Self> {
        /* This one now is done by Option::as_deref, so it exchanges the Box 
        with a reference */
        self.next.as_deref()
    }
    /* This function now needs to be mutable because we lost the Cell */
    pub fn set_next(&mut self, next: Option<Box<LinkedList1>>) -> Option<Box<LinkedList1>> {
        /* Not needed, as we could do two steps here. But I'll use replace anyways. */
        use std::mem::replace;
        replace(&mut self.next, next)
    }
    pub fn iter(&self) -> IterLinkedList1 {
        IterLinkedList1 {
            cursor: Some(&self),
        }
    }
    fn tail(&self) -> &Self {
        let mut cur = self;
        while let Some(next) = cur.next() {
            cur = next;
        }
        cur
    }

    /* And now we will need a mutable tail function, we lost Cell */
    fn _tail_mut_1(&mut self) -> &mut Self { 
        let mut cur = self;
        while let Some(next) = cur.next.as_deref_mut() {
            cur = next;
        }
        // cur  // returning this value requires that `cur.next` is borrowed for `'1`
        /* This happens because the borrow checker doesn't realize that "cur.next"
        cannot be the same "cur" as the end of the function */
        unimplemented!()
    }

    fn tail_mut(&mut self) -> &mut Self { 
        let mut cur = self;
        while let Some(curnext) = cur.next.as_deref_mut() {
            /* One trick to make it clear to the borrow checker is returning
            the value *before* putting it into cur. Not ideal code but we avoid
            using unsafe {} blocks. */
            if curnext.next.is_some() {
                cur = curnext;
            } else {
                return curnext;
            }
        }
        /* Here we need to tell the compiler that this code should never be
        executed. We actually never expect this to happen. That would be a bug.
        The "while let" it's really acting as a "loop {}" */
        unreachable!()
    }

    /* This one will need now to be using &mut self. Also the item instead of a
    reference we will taking the full value and claiming full ownership. This
    means the caller loses the value into the function. 
    
    For convenience I'll split this into two, one takes ownership, the other 
    takes already a box. This might be convenient for later.
     */
    fn insert_into(&mut self, item: LinkedList1) {
        let newnext = Box::new(item);        
        self.insert(newnext);
    }
    fn insert(&mut self, item: Box<LinkedList1>) {
        let oldnext = self.next.replace(item);
        /* Here because the mutable pointer is unique, we need to be smarter and
        realize that once the item is in our chain, its tail is actually now our
        tail, so just find our tail instead. This satisfies the borrow checker. */
        let tail = self.tail_mut();
        tail.next = oldnext;
    }

    fn replace(&mut self, item: Box<LinkedList1>, chain: bool) -> Option<Box<LinkedList1>> {
        let oldnext = self.next.replace(item);
        if chain {
            let tail = self.tail_mut();
            /* I had to do some weird descomposition in order to preserve 
            ownership. Not nice */
            if let Some(mut oldnext_val) = oldnext {
                if oldnext_val.next.is_some() {
                    tail.next = oldnext_val.next.take();
                } else {
                    tail.next = None;
                }
                Some(oldnext_val)
            } else {
                None
            }
        } else {
            oldnext
        }
    }

    fn append(&mut self, item: Box<LinkedList1>) {
        self.tail_mut().insert(item)
    }

    fn remove_next(&mut self) -> Option<Box<LinkedList1>> {
        let ret = self.next.take();
        /* Some(r) now needs to be mutable in order to perform r.next.take() */
        if let Some(mut r) = ret {
            let ret_next = r.next.take();
            self.next = ret_next;
            /* Instead of a common return, we compose it separately to avoid
            confusion for the borrow checker. This way it can clearly see that 
            the reference "r" is used only once, and ret is no longer used. */
            Some(r)
        } else {
            None
        }
    }
}
