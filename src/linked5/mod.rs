#![allow(dead_code)]
/*
Using `Rc<T>`to have prev and next pointers
===========================================================================

There's no other way in Rust to have the same pointer stored permanently.

The contents of Rc<T> are actually immutable, so it will return us a &T.
To allow the contents to be modified we need either a Cell<T> or RefCell<T>.

Which one do we need?

If we go the route "next: Rc<Cell<Node>>", then Node contains a
pointer, so it does not implement the Copy trait. This means we need to replace
the contents in order to read them. Bad for iterating.

If we go with "next: Rc<Cell<&Node>>", a lifetime is required. Which will
give us the same problem as with the first implementations. It will implement
copy, but not useful at all.

So the only sane way is going with "next: Rc<RefCell<Node>>"
*/
use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

struct Node {
    value: i64,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

pub struct List {
    first: Option<Rc<RefCell<Node>>>,
    tail: Weak<RefCell<Node>>,
}

impl Node {
    // NOTE: These implementations are not used at all!
    fn _new(value: i64) -> Self {
        Node {
            value,
            prev: None,
            next: None,
        }
    }
    fn _get_next(&self) -> Option<Ref<Node>> {
        self.next.as_ref().map(|x| x.borrow())
    }

    fn _tail(rcnode: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let rnode = rcnode.borrow();
        match &rnode.next {
            None => rcnode.clone(),
            Some(next) => Node::_tail(next.clone()),
        }
    }
}

impl Default for List {
    fn default() -> Self {
        Self {
            first: None,
            tail: Weak::new(),
        }
    }
}

impl List {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn slow_from_vec(v: &[i64]) -> Self {
        let mut l = Self::new();
        for n in v {
            l.append(*n);
        }
        l
    }

    pub fn from_vec(v: &[i64]) -> Self {
        if v.is_empty() {
            return List {first: None, tail: Weak::new()};
        }
        let mut nodes: Vec<Rc<RefCell<Node>>> = v
            .iter()
            .map(|n| Node {
                value: *n,
                prev: None,
                next: None,
            })
            .map(|n| Rc::new(RefCell::new(n)))
            .collect();
        for i in 0..nodes.len()-1 {
            nodes[i].borrow_mut().next = Some(nodes[i+1].clone());
            nodes[i+1].borrow_mut().prev = Some(nodes[i].clone());
        }
        List {
            first: Some(nodes[0].clone()),
            tail: Rc::downgrade(&nodes[nodes.len()-1]),
        }
    }

    pub fn to_vec(&self) -> Vec<i64> {
        self.iter().collect()
    }

    pub fn to_vec_rev(&self) -> Vec<i64> {
        self.iter().rev().collect()
    }

    pub fn concat(&mut self, other_list: List) {
        if other_list.first.is_none() {
            return;
        }
        let other = other_list.first.unwrap();
        if let Some(tail) = self.tail.upgrade() {
            let mut muttail = tail.borrow_mut();
            other.borrow_mut().prev = Some(tail.clone());
            self.tail = other_list.tail.clone();
            muttail.next = Some(other);
        } else {
            self.tail = other_list.tail.clone();
            self.first = Some(other);
        }
    }

    pub fn append(&mut self, value: i64) {
        let mut other = Node {
            value,
            next: None,
            prev: None,
        };

        if let Some(tail) = self.tail.upgrade() {
            let mut muttail = tail.borrow_mut();
            other.prev = Some(tail.clone());
            let otherref = Rc::new(RefCell::new(other));
            self.tail = Rc::downgrade(&otherref);
            muttail.next = Some(otherref);
        } else {
            let otherref = Rc::new(RefCell::new(other));
            self.first = Some(otherref.clone());
            self.tail = Rc::downgrade(&otherref);
        }
    }

    pub fn iter(&self) -> IterList {
        IterList {
            cursor: self.first.clone(),
            revcursor: self.tail.upgrade(),
        }
    }
}

pub struct IterList {
    cursor: Option<Rc<RefCell<Node>>>,
    revcursor: Option<Rc<RefCell<Node>>>,
}

impl Iterator for IterList {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.cursor.as_ref().map(|c| c.borrow().value);

        self.cursor = match self.cursor.as_ref() {
            Some(node) => {
                let reached_rcursor = if let Some(rnode) = self.revcursor.clone() {
                    use std::ops::Deref;
                    use std::ptr;
                    ptr::eq(rnode.deref(), node.deref())
                } else {
                    false
                };
                if reached_rcursor {
                    None
                } else {
                    let bnode = node.borrow();
                    bnode.next.clone()
                }
            }
            None => None,
        };
        ret
    }
}

impl DoubleEndedIterator for IterList {
    fn next_back(&mut self) -> Option<Self::Item> {
        let ret = self.revcursor.as_ref().map(|c| c.borrow().value);
        self.revcursor = match self.revcursor.as_ref() {
            Some(node) => {
                let reached_lcursor = if let Some(lnode) = self.cursor.clone() {
                    use std::ops::Deref;
                    use std::ptr;
                    ptr::eq(lnode.deref(), node.deref())
                } else {
                    false
                };
                if reached_lcursor {
                    None
                } else {
                    let bnode = node.borrow();
                    bnode.prev.clone()
                }
            }
            None => None,
        };
        ret
    }
}

#[cfg(test)]
mod test;
