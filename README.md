# mta_status
mtaStatus is a REST API that returns MTA subway status in Json format.

It is written in Rust for all the wonderful reasons, but most importantly the fact that it results in a tiny service. The tiny runtime cost really shines when you deploy it on a raspberryPi, which has become my main source for testing and personal projects.

## Why
The MTA is necessary if you live in NYC. It is also highly unreliable :| For this reason we need to keep an eye on which trains are running (especially for those early morning commutes).

- Now you have the option of using the wonderful widget that is provided by the MTA: `http://www.mta.info/mta-service-status-widget`
- Or maybe we like to view the status on our phones and the lack of a mobile site is annoying, so we look for an api: `http://web.mta.info/status/serviceStatus.txt`
- YES that is XML!! But wait is that HTML embedded inside the XML?? YES!

Therefore the nice Json API.

## Build
### Build for Raspi:
```
docker run \
    --volume $PWD:/home/cross/project \
    --volume /Users/toidiu/projects/rust-on-raspberry-pi:/home/cross/deb-deps \
    --volume ~/.cargo/registry:/home/cross/.cargo/registry \
    ragnaroek/rust-raspberry:1.17.0 build --release
```

copy executable onto your raspi
`scp -P 100 target/arm-unknown-linux-gnueabihf/release/mta_status pi@server.com:mta_status`

### Build for Docker on x86:
`make build`

## Other
### Some commands:
- `cargo install cargo-watch`
- `cargo watch -x 'rustc --bin mta-status --features clippy -- -Z no-trans'`   //assumes nightly(see Cargo.toml)
- `cargo watch -x 'rustc --lib --features clippy -- -Z no-trans'`              //assumes nightly(see Cargo.toml)
- `cargo watch -x run`
- `cargo build --release`

## Private Docs
cargo rustdoc --lib --open -- --no-defaults --passes collapse-docs --passes unindent-comments --passes strip-priv-imports

## Future work (PR's are welcome)
- add better logging
- add better error handling
- don't hit the MTA server on each request and instead cache the response in a local file
- implement the service as multi-threaded
- logo
- make a client to use the api :)
