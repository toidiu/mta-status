# Raspi:
```
docker run \
    --volume $PWD:/home/cross/project \
    --volume /Users/toidiu/projects/rust-on-raspberry-pi:/home/cross/deb-deps \
    --volume ~/.cargo/registry:/home/cross/.cargo/registry \
    ragnaroek/rust-raspberry:1.17.0 build --release
```

`scp -P 100 target/arm-unknown-linux-gnueabihf/release/mta_status pi@server.com:mta_status`



# Lunix:

- `cargo install cargo-watch`
- `cargo watch -x 'rustc --bin mta-status --features clippy -- -Z no-trans'`   //assumes nightly(see Cargo.toml)
- `cargo watch -x 'rustc --lib --features clippy -- -Z no-trans'`              //assumes nightly(see Cargo.toml)
- `cargo watch -x run`
- `cargo build --release`

# Docker:
make build


# Private Docs
cargo rustdoc --lib --open -- --no-defaults --passes collapse-docs --passes unindent-comments --passes strip-priv-imports
