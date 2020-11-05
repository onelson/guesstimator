FROM rust:1.47-buster as builder
# Stage 1

RUN apt-get update && \
    apt-get install --no-install-recommends -y \
    nodejs npm \
    && rm -rf /var/lib/apt/lists/*


ADD . /code

WORKDIR /code/

RUN cargo build --release -p phi-server
RUN cd phi-react && npm ci && npm run build

FROM debian:buster
# Stage 2

RUN mkdir /opt/phi
COPY --from=builder /code/target/release/phi-server /opt/phi/
COPY --from=builder /code/phi-react/build /opt/phi/dist
ENV PHI_STATIC_DIR=/opt/phi/dist
WORKDIR /opt/phi
EXPOSE 7878
CMD ["./phi-server"]