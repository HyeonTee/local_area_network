FROM ubuntu:22.04

RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    libcap-dev \
    pkg-config \
    arping \
    tcpdump \
	iputils-ping \
	net-tools \
	iproute2 \
	ncat \
	netcat \
	&& rm -rf /var/lib/apt/lists/*

ENV RUSTUP_HOME=/usr/local/rustup \
	CARGO_HOME=/usr/local/cargo \
	PATH=/usr/local/cargo/bin:$PATH

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal \
	&& rustup default stable \
	&& rustup component add rustfmt clippy \
	&& cargo install cargo-edit cargo-watch

WORKDIR /workspace

ENV USER=root
ENV HOME=/root