ARG RUST_VERSION=1.77
ARG APP_NAME=backend

FROM rust:${RUST_VERSION}-slim-bullseye AS build
LABEL author="dvorak"
LABEL email="dvorakchen@outlook.com"

ARG APP_NAME

WORKDIR /app

COPY ./backend/Cargo.toml ./backend/Cargo.toml
RUN mkdir ./backend/src
RUN echo "fn main() {}" > ./backend/src/main.rs
COPY ./share/Cargo.toml ./share/Cargo.toml
RUN mkdir ./share/src
RUN echo "" > ./share/src/lib.rs

RUN cargo build --release --manifest-path ./backend/Cargo.toml

RUN rm ./backend -rf
RUN rm ./share -rf

COPY ./backend ./backend
COPY ./share ./share

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
COPY --from=build /app/backend/epic.db /bin/epic.db

EXPOSE 8080

WORKDIR /bin

CMD ["server"]
