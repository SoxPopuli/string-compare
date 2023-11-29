FROM alpine:3.10 as build

COPY . /src/

RUN apk add nodejs yarn curl

# Install rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
#RUN apk add rustup
RUN rustup toolchain install nightly
ENV PATH="$HOME/.cargo/bin:${PATH}"

RUN cargo install wasm-pack

WORKDIR /src

RUN yarn run build
