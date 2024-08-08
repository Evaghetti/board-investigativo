FROM docker.io/rust:latest as builder
WORKDIR /
COPY . .

RUN rustup toolchain install nightly
RUN rustup override set nightly 
RUN rustup target add wasm32-unknown-unknown

RUN cargo install trunk
RUN trunk build --release

FROM docker.io/httpd:2.4
EXPOSE 80
COPY --from=builder /dist /usr/local/apache2/htdocs/
