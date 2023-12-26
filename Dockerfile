FROM rust:1.74.1 as builder

RUN USER=root cargo new --bin yt-dl-test
WORKDIR /yt-dl-test

# install cmake
RUN apt-get update && apt-get install -y cmake

# install ffmpeg
RUN apt-get install -y ffmpeg

COPY Cargo.toml Cargo.lock ./

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/yt_dl_test*
RUN cargo build --release

FROM debian:bookworm-slim as runtime

ENV DISCORD_TOKEN=$DISCORD_TOKEN

RUN apt-get update \
    && apt-get install -y \
    curl \
    python3

RUN ln -s libssl.so.3 libssl.so
RUN ldconfig

RUN curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o /usr/local/bin/yt-dlp
RUN chmod a+rx /usr/local/bin/yt-dlp

RUN apt-get clean && rm -rf /var/lib/apt/lists/*

COPY --from=builder /yt-dl-test/target/release/yt-dl-test .

CMD ["./yt-dl-test"]