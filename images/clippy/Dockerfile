ARG base_image=starlightpyro/rust-nightly:latest
FROM ${base_image}

RUN rustup component add clippy-preview

RUN cargo clippy
RUN rm src/*.rs

ENTRYPOINT ["/playground/tools/entrypoint.sh"]
