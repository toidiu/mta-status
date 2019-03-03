FROM scratch
ADD target/x86_64-unknown-linux-musl/release/mta-status /
# ADD target/debug/mta-status /
EXPOSE 4000
CMD ["/mta-status"]

