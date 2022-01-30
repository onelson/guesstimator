# FIXME: if we use musl, the final stage can be FROM SCRATCH
FROM rust:1.58-slim-bullseye as server-builder
ADD ./phi-server /code
WORKDIR /code/
RUN cargo build --release

FROM node:16-bullseye-slim as client-builder
ADD ./phi-react /code
WORKDIR /code/
RUN  npm ci && npm run build

FROM debian:bullseye-slim
ENV PHI_STATIC_DIR=/opt/phi/dist
RUN mkdir /opt/phi
COPY --from=server-builder /code/target/release/phi-server /opt/phi/
COPY --from=client-builder /code/build /opt/phi/dist
# FIXME: use ldd to verify no missing libs for binary (if not using musl).
WORKDIR /opt/phi
EXPOSE 7878
CMD ["./phi-server"]
