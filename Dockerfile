FROM rust:1.51 as builder

WORKDIR /root/

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --version 0.2.69 wasm-bindgen-cli

COPY . .

RUN cargo build --target wasm32-unknown-unknown --features web --release
RUN wasm-bindgen --out-dir wasm --target web target/wasm32-unknown-unknown/release/asteroids.wasm 


FROM nginx

RUN rm -rf /usr/share/nginx/html/*
COPY ./web /usr/share/nginx/html
COPY --from=builder /root/wasm /usr/share/nginx/html

RUN chmod -R +r /usr/share/nginx/html
