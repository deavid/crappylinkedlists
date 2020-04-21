Crappy Linked Lists in Rust
===============================

I wanted to learn proper Rust understand better lifetimes and stop avoiding
errors but instead looking for them. I want the compiler to blame at me and
explain why I'm so wrong. This is also known as learning the hard way.

Just on my first try I found out that Linked Lists is somewhat cursed when it
comes to borrow checking. Being a self-referrential data structure, it causes
the borrow checker to blame the programmer in almost every possible way.

Also, because Rust implements so many ways of holding data and data pointers,
Linked lists can be implemented in all sort of *wrong* ways. Each one can teach
us a different thing on the Rust borrow checker logic, and some magic we can do
to trick it (or not).

Linked lists are also (almost) the simplest self-referential struct, as they
only contain a value and a pointer in their simplest form:

```struct LinkedList {T, *LinkedList}```

Explore the different folders in src/ to see the different attempts to implement
those beast. I should *warn* you here: Almost every example is useless and
intentionally wrong. You've been warned.

- linked1: Value Only Linked Lists. 
  What happens if we refuse to use pointers at all costs?
- linked2: Reference Linked Lists.
  What if everything has to have a lifetime?
- linked3: Using `Cell<T>` to handle interior mutability of next
  So we get our Reference Linked Lists and tweak them so we can actually write.
- linked4: Adding Box to the mix
  And we should finally get something that hopefully works.

This is subtly based on another, better tutorial
-----------------------------------------------------

If you are into Rust you should know already about the Too Many Linked Lists
tutorial:

https://rust-unofficial.github.io/too-many-lists/index.html

I really recommend going through that one. It's very good. This one can be seen
as my own attempt of doing the same. Because coding is learned by practising,
these are my own practices. I skipped a lot of explanations of things that I 
already know, and I added my own thoughts (right or wrong) on why things are
like they are. Hope this makes sense.

Why is this code so bad? Doesn't make any sense
-----------------------------------------------------

It doesn't. But for self-referencing types like a linked list do expose a lot
of problems with the borrow checker. The easy way handling this would be going
for a `Rc<RefCell<T>>` and forget. But Rc is close to garbage collectors, and
RefCell also adds an extra write for borrowing.

I wanted to explore how far we can get with as little extra tools as possible,
doing as much as possible at compile time. Because, programs that are valid on
compile time and don't do checks in runtime should have better guarantees of
working properly in any situation. Also, they should be faster too.

This is a learning experience, at least for me. Learning how to do complex stuff
with the borrow checker without resorting to the "easy mode" should allow me to
write better Rust programs in the future, understanding better the errors, and
why Rust compiler behaves the way it does.

Also this is a good training to know what are the different primitives to manage
memory and which one is best for each scenario.

Reference to the different modes of storing data in Rust
==========================================================

Values
-------

When we store values directly, those are copied or moved onto their places.
For short and simple datatypes this is convenient. It takes the same to copy
an i64 than to write a pointer to it (because they're the same size).

However, on a struct copying it would mean to do several CPU cycles to write on
the memory. If the struct is small this is acceptable. If it's big, it will take
a lot of time on each write. And those copies happen usually on reads as well.

In those cases you should return a reference instead, so the value cannot be 
copied.

References
-------------

Storing references is tricky because they have a lifetime and you have to prove
for the whole lifetime of the program that the pointer would be valid.

This implies that some sort of lifetime declaration has to be made and when
variables are created locally they're destructed on function exit.

Mutable references are even tricker because you can only have one, and during
that time, no immutable references are allowed.

Option
--------

When storing references it's easy to notice that Rust does not allow NULL 
pointers, so usually it's hard to create structures that contain bare references.

This is when Option comes handy as it allows for nullable pointers. Although the
construction might look complex, it doesn't take extra space for pointers. So
`Option<&DataStruct>` has the same size as `&DataStruct` and takes the same CPU
cycles to dereference or construct.

Rust has a lot of boilerplate that tells the compiler how to understand the
program and prove it correct, but it doesn't add any extra computation or data.

For non-references like `Option<i64>`, the Option will use an extra u64 
(actually more like an u8 with memory padding for alignment). But nonetheless,
this cannot be done cheaper in any other language.

Cell
------------

`Cell<T>` is a cheap way to make a struct member mutable even if we got an
immutable reference to the struct. This is what is called interior mutability.

Basically it isolates the Cell contents from the struct, so it has its different
rules for borrowing and mutability.

Cell it's not a holy grail, it doesn't add any runtime cost but has other 
compile time limitations. Namely, in order to read the contents you must copy
them. If the contents are not copiable, then you have to destroy the contents
to read them either by take() or replace().

Because of this, Cell is most useful with types that are actually copiable.
For example types like u8 and i64 are interesting because they only take one
CPU cycle to copy. Vectors are not copiable and arrays might be too big to
be useful. But, turns out that reference pointers are always copiable (&x), so
if you got a complex, non-copyable structure, the reference pointer can always
be copied over in a single CPU cycle because its size is `usize`.

This will not give you any grant to mutate the interior data of the struct that
you're referencing (of course internals can be changed if they're wrapped in a
Cell), but it allows you to swap the pointer with another.

In short, having an immutable reference isn't a total guarantee that you will
not be able to change it. There are ways to change the contents if the correct
types are used.
