FROM rust:alpine AS chef
RUN apk add pkgconfig openssl-dev libc-dev
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM eclipse-mosquitto:2.0.18
ARG S6_OVERLAY_VERSION=3.2.0.0

ADD https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-noarch.tar.xz /tmp
RUN tar -C / -Jxpf /tmp/s6-overlay-noarch.tar.xz
ADD https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-x86_64.tar.xz /tmp
RUN tar -C / -Jxpf /tmp/s6-overlay-x86_64.tar.xz

COPY --from=builder /app/target/release/fake-c8y /usr/bin/fake-c8y

RUN mkdir -p /etc/s6-overlay
COPY ./s6-rc.d /etc/s6-overlay/s6-rc.d
# RUN mkdir -p /etc/services.d/mosquitto
# RUN echo -e "#!/command/execlineb -P\nmosquitto -c /mosquitto/config/mosquitto.conf" > /etc/services.d/mosquitto/run

RUN chmod +x /etc/s6-overlay/s6-rc.d/mosquitto/run

ENTRYPOINT [ "/init" ]
# CMD [ "fake-c8y" ]