FROM rust:1.97-bookworm

RUN apt-get update \
    && apt-get install --yes z3 \
    && rm -rf /var/lib/apt/lists/*
