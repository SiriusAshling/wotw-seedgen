FROM rust:alpine as build-seedgen

COPY . /app
WORKDIR /app

RUN apk --no-cache add musl-dev && \
    cargo build --release --target-dir /app/build/output


FROM alpine

WORKDIR /app

COPY --from=build-seedgen /app/target/release/seedgen /app/seedgen
COPY --from=build-seedgen /app/wotw_seedgen/headers /app/headers
COPY --from=build-seedgen /app/wotw_seedgen/presets /app/presets
COPY --from=build-seedgen /app/wotw_seedgen/areas.wotw /app/areas.wotw
COPY --from=build-seedgen /app/wotw_seedgen/loc_data.csv /app/loc_data.csv
COPY --from=build-seedgen /app/wotw_seedgen/state_data.csv /app/state_data.csv
