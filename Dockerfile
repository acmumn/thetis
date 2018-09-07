FROM clux/muslrust:stable
RUN apt-get update && apt-get install -y \
  libmysqlclient-dev \
  --no-install-recommends && \
  rm -rf /var/lib/apt/lists/*
ADD . /volume
RUN cargo build --release

FROM alpine:latest
# RUN apk add --no-cache TODO mysql-client-libs
COPY --from=0 /volume/target/x86_64-unknown-linux-musl/release/thetis /usr/local/bin/thetis
CMD /usr/local/bin/thetis
