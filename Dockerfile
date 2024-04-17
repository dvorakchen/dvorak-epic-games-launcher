ARG RUST_VERSION=1.77
ARG APP_NAME=backend

FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME

WORKDIR /app

COPY ./backend ./backend
COPY ./share ./share

# WORKDIR /app/backend

RUN --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=cache,target=/app/backend/target/

RUN cargo build --release --manifest-path ./backend/Cargo.toml

FROM debian:bullseye-slim AS final

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

COPY --from=build /app/backend/target/release/backend /bin/server

EXPOSE 8080

CMD ["/bin/server"]
