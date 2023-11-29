FROM alpine:3.18.4 as build

COPY . /src/

RUN apk add nodejs yarn curl g++ make

# Install rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
#RUN apk add rustup
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup toolchain install nightly
RUN rustup default nightly
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-pack

WORKDIR /src

RUN rm -rf node_modules || true
RUN yarn install --immutable
RUN yarn run build

CMD [ "cp", "-rv", "/src/dist", "/github/workspace/" ]
