FROM rust:slim-bookworm AS build
WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY geolocation ./geolocation
RUN sed -i 's/^members = .*/members = ["geolocation"]/' Cargo.toml \
    && cargo build --release -p compallm-geolocation

FROM debian:bookworm-slim AS data
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY geolocation/download-dbip.sh /app/geolocation/download-dbip.sh
RUN /app/geolocation/download-dbip.sh

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /build/target/release/compallm-geolocation /usr/local/bin/compallm-geolocation
COPY --from=data /app/artifacts/geoip /app/artifacts/geoip
USER 65534:65534
ENV PORT=8080
EXPOSE 8080
CMD ["compallm-geolocation"]
