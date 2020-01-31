FROM ekidd/rust-musl-builder:latest AS builder
COPY . .
RUN cargo build --release --target=x86_64-unknown-linux-musl
RUN strip target/x86_64-unknown-linux-musl/release/overengineer-dis-backend

FROM scratch
COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/overengineer-dis-backend /overengineer-dis-backend
CMD ["/overengineer-dis-backend"]
