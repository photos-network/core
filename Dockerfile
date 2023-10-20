FROM rust:latest AS builder
LABEL "description"="Photos.network core system"
LABEL "version"="0.6.0"
LABEL "maintainer"="github.com/photos-network"

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
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

RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /core

# Copy our build
COPY --from=builder /core/target/x86_64-unknown-linux-musl/release/core ./

# Use an unprivileged user.
USER core:core

CMD ["/app/core"]
