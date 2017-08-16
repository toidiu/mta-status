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




Understand the Core of Rust
- green threading
- tokio
- mio

Ownership
- https://play.rust-lang.org/
