#![allow(dead_code)]
/*
Reference Linked Lists
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
    next: Option<&'a LinkedList2<'a>>,
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
        LinkedList2 { value, next }
    }

    /* Some getters and setters for public access: */
    pub fn value(&self) -> i64 {
        self.value
    }
    pub fn set_value(&mut self, value: i64) {
        self.value = value;
    }
    pub fn next(&self) -> Option<&Self> {
        self.next
    }
    pub fn set_next(&mut self, next: Option<&'a LinkedList2<'a>>) {
        self.next = next;
    }
}
/*
Now onto interesting stuff, how do we implement an iterator?

We will need a struct that can hold the current position or we'll consume
our own items while iterating. Because of this, we will need a function
that returns one of those iterable structs:
*/
pub struct IterLinkedList2<'a> {
    cursor: Option<&'a LinkedList2<'a>>,
}

/* And now we implement a iter() function that returns this struct: */
impl<'a> LinkedList2<'a> {
    pub fn iter(&self) -> IterLinkedList2 {
        IterLinkedList2 {
            cursor: Some(&self),
        }
    }
}

/* And the iterator. We need to implement the trait */
impl<'a> Iterator for IterLinkedList2<'a> {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        /* We get the return value. Using map() we can translate from
        Option<LinkedList> to Option<c.value(i64)> */
        let ret = self.cursor.map(|c| c.value);
        /* Now we have to advance the cursor to the next item. Flatten is used
        to remove the Option<Option<T>> and leave a single one. */
        self.cursor = self.cursor.map(|c| c.next).flatten();
        ret
    }
}

/* So far so good. Let's add functionality to add, remove, etc... */
impl<'a> LinkedList2<'a> {
    /* We will need first a method that finds the tail */
    fn tail(&self) -> &Self {
        let mut cur = self;
        while cur.next.is_some() {
            cur = cur.next.unwrap()
        }
        cur
    }
    /* Here's our first problem: we can't do a mutable version of tail because
    the reference in next is not mutable itself: */
    // fn tail_mut(&mut self) -> &mut Self {
    //     let mut cur = self;
    //     while cur.next.is_some() {
    //         cur = cur.next.unwrap()
    //     }
    //     cur
    // }

    /* Insert is more complicated. We want to insert after this item */
    fn insert(&mut self, item: &'a mut LinkedList2) -> Option<&Self> {
        /* first switch our next with that item */
        let oldnext = self.next.replace(item);
        /* now we need to add the remaining part of the list at the end */
        let _tail = item.tail();
        /* And fails again, because tail() is not mutable, we can't add
        ourselves there: */
        // tail.next.replace(oldnext);

        /* So the only option is to return the reference and let the caller
        do it: */
        oldnext
    }
}

/*
I will stop here because I believe this is already a complete disaster. It's
very inconvenient for the caller to do the other half of the operations.

So, the property next needs to be mutable? That doesn't work. There can be only
one proved mutable reference to any part at compile time. These restrictions
backfire in these structures:
*/
struct LinkedList3<'a> {
    value: i64,
    next: Option<&'a mut LinkedList3<'a>>,
}

impl<'a> LinkedList3<'a> {
    pub fn new(value: i64, next: Option<&'a mut LinkedList3<'a>>) -> Self {
        LinkedList3 { value, next }
    }
    pub fn value(&self) -> i64 {
        self.value
    }
    pub fn set_value(&mut self, value: i64) {
        self.value = value;
    }
    pub fn next(&self) -> Option<&Self> {
        self.next.as_deref()
    }
    pub fn set_next(&mut self, next: Option<&'a mut LinkedList3<'a>>) {
        self.next = next;
    }
    fn tail(&self) -> &Self {
        let mut cur = self;
        while cur.next.is_some() {
            cur = cur.next.as_deref().unwrap()
        }
        cur
    }
    /* Let's go for tail_mut again! */
    // fn tail_mut(&mut self) -> &mut Self {
    //     let mut cur = self;
    //     while cur.next.is_some() {
    //         /* cannot move out of `cur.next` which is behind a mutable reference */
    //         cur = cur.next.unwrap()
    //     }
    //     cur
    // }

    /*
    Dang! this is quite unfixable. Because it's inside the struct, reading
    the pointer will imply that there will be two pointers now, both mutable.

    The only way around is a nasty one, removing the pointer as we iterate. In
    this way there will be one and only one pointer to the same place at once.

    In order to return a mutable reference, we will need to remove the reference
    from the previous leaf and leave None. The caller would need to put it back.

    But, to put back the reference to the now last tail, the caller would need
    to call this function again, cleaning a new item. So in short, this means
    the caller MUST recreate the entire chain in order to change a single item.

    Not cool.
    */
    fn tail_mut(&mut self) -> &mut Self {
        let mut prev: Option<&mut Self> = None;
        let cur = self;
        while let Some(next) = cur.next.take() {
            /* We have to pacify the borrow checker doing both operations at once:
            first, put the next value onto cur, so we advance, but also store the
            old value somewhere (next) so we can access it later. */
            std::mem::swap(&mut cur.next, &mut prev);
            std::mem::swap(cur, next);
            prev.replace(next);
        }
        cur
    }
}
/*
Does the above code work? I'm not sure, and not even going to bother proving it.
Even if it works as intended (Rust seems happy about it) it's going to be a
really stupid idea.

If you have to re-do the whole list, just redo it from scratch.
It's going to be simpler.

So, if the "next" is read-only there's a big problem. If it's mutable we have
even a bigger problem. So what now?

Let me advise that the solution is to use an Rc<T> for simplicity, or at least
a Box<T>. We'll see those later, we still have some fun here.

Can we fix this?

Well, the root of the problem is that someone has to hold the mutable data or
its ownership. Because LinkedList3 is sparse, the ownership is hard to account
to whom it belongs.

For read methods, the sparse structure is fine. But for write ones, we could
leverage a centralized one where the data is hold.

This is going to be the worse implementation ever of LinkedList, closely
followed by the monster of stack-only-recursive-generics we did earlier.

Why? Because we will need to use an array or vector to hold the actual data.
Is this a linked list anymore?
*/

type Node4<'a> = LinkedList2<'a>;

struct LinkedList4<'a> {
    pub first: Option<&'a Node4<'a>>,
    pub data: Vec<Node4<'a>>,
}

/*
I've used a vector and not an array which is a dynamic structure. It would be
"better" to use static arrays to show that we're actually managing our data
and not simply letting others to do it for us. And also it would be amazing to
see them concatenated together, so you could create longer ones by chaining them.

But I want to retain my sanity (up to a point), and I hope you'll agree here.

I've aliased LinkedList2 to Node4, because it's the same code, same data.
There's no need to repeat ourselves.

You might ask, why the data is Vec<Node4> and not Vec<&mut Node4>. Both should
work, but the latter implies that the caller still has ownership and has to
handle memory allocation; then we would have our vector of pointers. This adds
another layer of complexity that is not needed. It's simpler to everyone if
we just own and hold it inside.

Another question here is, how this schema is going to make take_mut() easier to
code. That's because Rust borrow checker is going to "lock" the whole list
when we try to obtain mutable access to anything. So, from this list, only
a single item is going to be able to be changed at a time.

If we wanted to be able to hold several mutable pointers at once we should wrap
the actual values into RefCell<T>, this structure is a non-thread lock that can
allow many readers and one writer at runtime, panicking if that's not met. It's
really cheap, because it only increments or decrements a number when the lock
is obtained or released. But then the "next" inside Node4 must also be a RefCell
because if not you'll never know if someone is reading while you're writting.

And then, this approach is not that needed anymore, because you could mutate
from the nodes themselves. There's not much need for a structure like this for
holding ownership and mutability anymore; of course it is always helpful to have
a wrapper around the nodes, but that's a different story.

We'll see a RefCell version later, for now let's get crazy once again:
*/
impl<'a> LinkedList4<'a> {
    fn new() -> Self {
        LinkedList4 {
            first: None,
            data: vec![],
        }
    }
    fn tail(&self) -> Option<&Node4> {
        self.first.map(|f| f.tail())
    }
    fn tail_idx(&mut self) -> Option<usize> {
        match self.tail() {
            None => None,
            Some(tail) => {
                for (i, n) in self.data.iter().enumerate() {
                    if n as *const Node4 == tail as *const Node4 {
                        return Some(i);
                    }
                }
                None
            }
        }
    }

    fn append(&'a mut self, value: i64) {
        self.data.push(Node4 { value, next: None });
        if let Some(_i) = self.tail_idx() {
            let _last = self.data.last();
            //         ----------------
            //         |
            //         immutable borrow occurs here
            //         argument requires that `self.data` is borrowed for `'a`
            // if let Some(mut tail) = self.data.get_mut(i) {
            //     tail.next = last;
            // }
        }
    }
}
/*
At least to me, this looks impossible to fix. The borrow checker is not going
to prove that data.last() and data.get_mut() do not retrieve the same address.
Also, the borrow checker borrows the full data vector, so basically it's not
possible to write to it while reading from it.

Storing that reference elsewhere isn't going to help either. Even if you manage
to create a second Vec<&Node4> and put all the references there, the original
Vec<Node4> would be locked for read-only the whole time. (I don't think this is
even possible to do)
*/
