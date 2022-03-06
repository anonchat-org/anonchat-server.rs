# --- Builder ---
FROM rust:latest AS builder

RUN update-ca-certificates

# Create appuser
ENV USER=anonchat-server
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /anonchat-server

COPY ./ .

RUN cargo build --release


# --- Final Image ---
FROM debian:buster-slim

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /anonchat-server

# Copy our build
COPY --from=builder /anonchat-server/target/release/anonchat-server ./

# Use an unprivileged user.
USER anonchat-server:anonchat-server

ENTRYPOINT ["/anonchat-server/anonchat-server"]
