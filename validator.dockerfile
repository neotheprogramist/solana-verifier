FROM rust:bookworm

RUN sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
ENV PATH="/root/.local/share/solana/install/active_release/bin:$PATH"

ENTRYPOINT [ "solana-test-validator", "--compute-unit-limit", "10000000000000" ]
