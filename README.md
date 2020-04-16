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
- linked2: Stack-pointer Linked Lists.
  What if everything has to have a lifetime?