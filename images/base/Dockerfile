FROM ubuntu:20.04 as toolchain

ARG channel

ENV DEBIAN_FRONTEND="noninteractive"

RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    curl \
    file \
    gcc \
    git \
    libssl-dev \
    pkg-config \
    python \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m playground -d /playground
RUN usermod -p '!!' root
USER playground
ENV USER=playground
ENV PATH=/playground/.cargo/bin:$PATH
WORKDIR /playground

ADD --chown=playground https://static.rust-lang.org/rustup/release-stable.toml /playground/tools/rustup-manifest.toml
ADD --chown=playground https://static.rust-lang.org/dist/channel-rust-${channel}-date.txt /playground/tools/rust-channel-version
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain "${channel}"

ADD --chown=playground entrypoint.sh /playground/tools/

FROM toolchain as bare-sources

RUN cargo init /playground

ADD --chown=playground ./Cargo.toml /playground/Cargo.toml
RUN cargo fetch

FROM bare-sources as munge

ADD --chown=playground modify-cargo-toml /playground/modify-cargo-toml
RUN cargo build --release --manifest-path=/playground/modify-cargo-toml/Cargo.toml

FROM bare-sources as sources

COPY --from=munge /playground/modify-cargo-toml/target/release/modify-cargo-toml /playground/.cargo/bin/

FROM sources

ARG channel

RUN cargo build
RUN cargo build --release
RUN rm src/*.rs

ADD --chown=playground postinstall.sh /playground/tools/
RUN /playground/tools/postinstall.sh ${channel}
ADD --chown=playground cargo-wasm /playground/.cargo/bin/

ENTRYPOINT ["/playground/tools/entrypoint.sh"]
