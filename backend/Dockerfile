
ARG RUST_VERSION=1.74.0
ARG APP_NAME=backend
FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app

# Build the application.
# Leverage a cache mount to /usr/local/cargo/registry/
# for downloaded dependencies and a cache mount to /app/target/ for 
# compiled dependencies which will speed up subsequent builds.
# Leverage a bind mount to the src directory to avoid having to copy the
# source code into the container. Once built, copy the executable to an
# output directory before the cache mounted /app/target is unmounted.
RUN --mount=type=bind,source=backend/src,target=src \
    --mount=type=bind,source=backend/Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=backend/Rocket.toml,target=Rocket.toml \
    --mount=type=bind,source=backend/Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --release
cp ./target/release/$APP_NAME /bin/server
EOF

FROM debian:bullseye-slim AS final

# See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

RUN mkdir /var/blogs
COPY ../content/blogs/* /var/blogs/
RUN chown -R appuser /var/blogs
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

EXPOSE 8000
CMD ["/bin/server"]