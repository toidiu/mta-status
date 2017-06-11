docker run \
    --volume /Users/toidiu/projects/mtaStatus:/home/cross/project \
        --volume /Users/toidiu/projects/rust-on-raspberry-pi:/home/cross/deb-deps \
            --volume ~/.cargo/registry:/home/cross/.cargo/registry \
                ragnaroek/rust-raspberry:1.17.0 build --release
