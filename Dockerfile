FROM rust:latest AS builder
LABEL "description"="Photos.network core system"
LABEL "version"="0.6.0"
LABEL "maintainer"="github.com/photos-network"

ARG TARGETARCH

RUN case "$TARGETARCH" in \
      amd64) echo "x86_64-unknown-linux-musl"   > /rust_target ;; \
      arm64) echo "aarch64-unknown-linux-musl"  > /rust_target ;; \
    esac

RUN rustup target add $(cat /rust_target)
RUN apt-get update && apt-get install -y musl-tools musl-dev && rm -rf /var/lib/apt/lists/*
RUN update-ca-certificates

ENV USER=core
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /core

COPY ./ .

RUN cargo build --target $(cat /rust_target) --release && \
    cp target/$(cat /rust_target)/release/core /core_bin

####################################################################################################
## Final image
####################################################################################################
FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /core

# Copy our build
COPY --from=builder /core_bin ./core

# Use an unprivileged user.
USER core:core

CMD ["/core/core"]
