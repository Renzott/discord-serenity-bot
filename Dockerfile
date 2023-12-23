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

RUN apt-get update && apt-get install -y ffmpeg
RUN apt-get install -y libssl-dev
RUN apt-get install -y ca-certificates
RUN apt-get install -y software-properties-common
RUN apt-get install -y python3-launchpadlib

RUN ln -s libssl.so.3 libssl.so
RUN ldconfig

RUN add-apt-repository ppa:tomtomtom/yt-dlp
RUN apt-get update
RUN apt-get install -y yt-dlp

COPY --from=builder /yt-dl-test/target/release/yt-dl-test .

CMD ["./yt-dl-test"]