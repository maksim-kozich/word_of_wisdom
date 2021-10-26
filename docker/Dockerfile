FROM rust:1.55 as builder
WORKDIR /usr/src/word_of_wisdom
COPY . .
RUN cargo build --release

FROM rust:1.55 as app
#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/word_of_wisdom/target/release/wow-server /usr/local/bin/wow-server
COPY --from=builder /usr/src/word_of_wisdom/target/release/wow-client /usr/local/bin/wow-client