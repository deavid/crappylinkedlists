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
pub struct LinkedList1 {
    pub value: i64,
    pub next: Option<Box<LinkedList1>>,
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
    /* This will come handy sometime later */
    pub fn new_box(value: i64, next: Option<Box<LinkedList1>>) -> Box<Self> {
        Box::new(LinkedList1 {
            value,
            next,
        })
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
    pub fn tail(&self) -> &Self {
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
    pub fn insert_into(&mut self, item: LinkedList1) {
        let newnext = Box::new(item);        
        self.insert(newnext);
    }
    pub fn insert(&mut self, item: Box<LinkedList1>) {
        let oldnext = self.next.replace(item);
        /* Here because the mutable pointer is unique, we need to be smarter and
        realize that once the item is in our chain, its tail is actually now our
        tail, so just find our tail instead. This satisfies the borrow checker. */
        let tail = self.tail_mut();
        tail.next = oldnext;
    }

    pub fn replace(&mut self, item: Box<LinkedList1>, chain: bool) -> Option<Box<LinkedList1>> {
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

    pub fn append(&mut self, item: Box<LinkedList1>) {
        self.tail_mut().insert(item)
    }

    pub fn remove_next(&mut self) -> Option<Box<LinkedList1>> {
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

/*
Success at last! It took four versions but this one is the first functional one.
Still, we cannot hold zero items, but that's not a big deal. Anyway, we'll 
create a wrapper class to manage this state.
*/

/*
I want to go crazy again. So this manager only has two states, either empty or
a list of 1 or more items. Can we code this as a Rust enum?
*/

/* Was going to use "None", but for practice, I guess we can reinvent the wheel */
pub enum List {
    First(Box<LinkedList1>),
    Empty,
}

impl List {
    pub fn new_slow(slice: &[i64]) -> Self {
        let mut iter = slice.iter();
        /* Because we don't know the length of the slice, the only way to get
        the 1st value, then the remaining, is to use an iterator. Consume the
        first item, then iterate the remaining. */
        let opt_value = iter.next();
        if opt_value.is_none() {
            return List::Empty;
        }
        let value = opt_value.unwrap();
        /* value needs to be de-referenced to do a copy, since i64 implements
        copy, this is possible. If the type instead of i64 was non-copyable, we
        would have to choose, either .clone() it (if it allows) or take full
        ownership. */
        let mut first = LinkedList1::new_box(*value, None);
        for value in iter {
            /* This is not really efficient as it will iterate the list each time */
            first.append(LinkedList1::new_box(*value, None))
        }
        List::First(first)
    }
    /* Let's try a faster version */
    pub fn new_bad(slice: &[i64]) -> Self {
        let mut iter = slice.iter();
        let opt_value = iter.next();
        if opt_value.is_none() {
            return List::Empty;
        }
        let value = opt_value.unwrap();
        let mut first = LinkedList1::new_box(*value, None);
        let mut cur = &mut first;
        for value in iter {
            cur.next = Some(LinkedList1::new_box(*value, None));
            /* this doesn't seem possible because Rust thinks we have access now
            to two pointers at the same time */
            // cur = &mut cur.next.unwrap();
            unimplemented!();
        }
        List::First(first)
    }
    /* We need to construct it backwards, from tail to head... */
    pub fn new(slice: &[i64]) -> Self {
        let mut cur = None::<Box<LinkedList1>>;
        for elem in slice.iter().rev() {
            let mut new = LinkedList1::new_box(*elem, None);
            if let Some(prev) = cur {
                new.next = Some(prev);
            }
            cur = Some(new);
        }
        match cur {
            Some(list) => List::First(list),
            None => List::Empty,
        }
    }
    /* We'll try a simply add_item... */
    pub fn add_item(&mut self, value: i64) {
        let new = LinkedList1::new_box(value, None);
        if let List::First(list) = self {
            let mut tail = list.tail_mut();
            tail.next = Some(new);
        } else {
            // This feels strange. We can "replace" the contents just by
            // de-referencing. I was expecting this to fail:
            *self = List::First(new);
        }
    }

    pub fn tail_mut(&mut self) -> Option<&mut LinkedList1> {
        match self {
            List::First(list) => Some(list.tail_mut()),
            List::Empty => None,
        }
    }

    /* let's try a concatenate! We will copy the values as we iterate. */
    pub fn concat_copy(&mut self, other: &Self) {
        if let List::First(list) = other {
            /* in order to do this efficiently we should create it in reverse
            order, as doing tail each time would be a waste: */
            /*for elem in list.iter() {
                // add_item does tail_mut, so we're iterating each time. Bad.
                self.add_item(elem);
            }*/
            /* Can we iterate in reverse?? */
            // for elem in list.iter().rev() {
            //     //      ^^^ the trait `std::iter::DoubleEndedIterator` is not implemented for `linked4::IterLinkedList1<'_>`
            // }
            
            /* Turns out that for this we would need the full array anyway, so ... */
            let array: Vec<i64> = list.iter().collect();
            let mut cur: Option<LinkedList1> = None;
            for elem in array.iter().rev() {
                cur = Some(LinkedList1::new(*elem, cur.map(Box::new)))
            }
            let boxval = cur.map(Box::new);
            /* TODO: Add comments here... it's quite complex. */
            match self {
                List::First(self_list) => {
                    let tail = self_list.tail_mut();
                    tail.next = boxval;
                },
                List::Empty => {
                    if let Some(v) = boxval {
                        *self = List::First(v);
                    }
                }
            }


        }
    }
}
