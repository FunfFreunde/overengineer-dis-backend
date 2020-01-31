FROM ekidd/rust-musl-builder:latest AS builder
COPY . .
RUN cargo build --release --target=x86_64-unknown-linux-musl
RUN strip target/x86_64-unknown-linux-musl/release/overengineer-dis-backend
COPY target/x86_64-unknown-linux-musl/release/overengineer-dis-backend /

FROM scratch
COPY --from=builder /overengineer-dis-backend /overengineer-dis-backend
CMD ["/overengineer-dis-backend"]
