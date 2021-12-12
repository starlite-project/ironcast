ARG base_image=starlightpyro/rust-nightly:latest
FROM ${base_image}

RUN rustup component add rustfmt-preview

ENTRYPOINT ["/playground/tools/entrypoint.sh"]
