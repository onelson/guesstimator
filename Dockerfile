FROM rust:1.58-slim-bullseye as server-builder
ADD ./ /code
WORKDIR /code/
RUN cargo build --release

FROM node:16-bullseye-slim as client-builder
ADD ./phi-react /code
WORKDIR /code/
RUN  npm ci && npm run build

FROM debian:bullseye-slim
RUN mkdir /opt/phi
COPY --from=server-builder /code/target/release/phi-server /opt/phi/
COPY --from=client-builder /code/build /opt/phi/dist
ENV PHI_STATIC_DIR=/opt/phi/dist
WORKDIR /opt/phi
EXPOSE 7878
CMD ["./phi-server"]