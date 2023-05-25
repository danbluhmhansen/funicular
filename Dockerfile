FROM rust:1.69.0 AS build
WORKDIR /funicular
COPY . .
RUN apt update
RUN apt install -y clang
RUN cargo install --locked cargo-pgrx
RUN cargo pgrx init
RUN cargo pgrx package --pg-config /root/.pgrx/15.3/pgrx-install/bin/pg_config --out-dir .

FROM postgres:15
COPY --from=build /funicular/root/.pgrx/15.3/pgrx-install/share/postgresql/extension/* /usr/share/postgresql/15/extension/
COPY --from=build /funicular/root/.pgrx/15.3/pgrx-install/lib/postgresql/funicular.so /usr/lib/postgresql/15/lib/
