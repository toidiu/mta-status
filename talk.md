# Content
- mio overview
- tokio overview
- creating a server
- future api
- bin vs lib
- mod files/code organization
- Box vs Map
- exposing lib project (small api surface)
- pass by reference (prefered)
- building for raspi/linux
- thread sleep example


### dev tooling
- clippy
- fmt
- cargo watch
- #![deny(warnings)] vs #![allow(unused)]

# Topic

# Structure



The talk can have multiple parts to it. I am going to list a few:
- development pattern one should be using to be highly productive
  - clippy, fmt, cargo watch, allow unused
- how to organizing and structure your code
  - mod, lib vs bin, pub vs private
  - multi-project projects.. https://github.com/paritytech/jsonrpc/blob/master/Cargo.toml
- building production-ready programs
  - concurrency
  - error handling
  - future api
  - an explanation into tokio and mio
  - limitations of the program (single threaded)
  - managing the core/handle (1 core per thread)
  - green threading vs event loop


The philosphy behind rust: memory safety, performance,




* Understand the Core of Rust
- green threading
- tokio
- mio

Running work async is important. to  do this we need libraries such as tokio and mio. these
libraries make up the core of alot of other libraries and yet they remain a black box to
many. the goal is to gain better insight into these libraries and therefore a better
understanding of how to use them to create async programs.

- to accomplish this i will need to intro the green threading model
- then talk about Rust's lack of it
- then talk about event loop, epoll
- then talk about futures
- then talk about the single threaded nature of core

* Ownership at the core of API design
This is very good topic because it hits on the part of Rust that differentiates it from other
languages and also the feature that gives it power.

- to accomplish this i will need to intro ownership, lifetimes and borrow concepts.
- the second step will be to then think about these concepts when designing an API.
  - ownership stems from `copy` and what it means in Rust. semantic vs byte copy
  - copy (byte vs semantic) is a copy of the reference or underlying data. by default it is always a byte copy
  - there is the concept of move, & and mut& <https://stackoverflow.com/questions/24253344/move-vs-copy-in-rust>
  - by-value (not reference) use in Rust is always a byte copy <https://github.com/rust-lang/rfcs/blob/master/text/0019-opt-in-builtin-traits.md#controlling-copy-vs-move-with-the-copy-trait>
  - move means no longer accessing it
  - & is on the stack and means sharing the same mem location(for primitive types this means a semantic copy but for others it means a reference copy/move)
  - mut& is to share the same mem for the reference to mutate the data underneath


===========
  - what is a copy in Rust
  - difference between byte and semantic copy
  - how the different types of copy can cause errors
  - how move/ownership semantic solves this issue
  - explain rules of owenership
  - what this gets you (compile time safety, runtime performance, fearless concurrency)

  - there are 3 ways to share data: & and mut& and move
  - reference laws include:
    - & can only last as long as the owner
    - you can have as many & as you want
    - you can only have 1 mut&

  - implications of ownership on API design
    - restrict when to invalidate data in context (move)
    - share reas-only/immutable data with many (&)
    - allow for changes in data (&mut)
===========

  - why we need ownership: because byte copies means we can modify the same underlying data from two places
    - alternatives include to let the programmer handle it or always do a semantic copy


  - api design can mean that we can return self, &self, &mut self
    
