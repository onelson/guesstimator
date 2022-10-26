FROM node:16-bullseye-slim as client-builder
ADD ./phi-react /code
WORKDIR /code/
RUN  npm ci && npm run build

FROM ekidd/rust-musl-builder:stable as server-builder
ADD ./phi-server /home/rust/src
COPY --from=client-builder /code/build /home/rust/src/frontend
ENV PHI_STATIC_DIR=/home/rust/src/frontend
RUN cargo build --release --features baked


FROM scratch
COPY --from=server-builder /home/rust/src/target/x86_64-unknown-linux-musl/release/phi-server /
EXPOSE 7878
ENTRYPOINT ["/phi-server"]
