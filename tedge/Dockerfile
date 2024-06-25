FROM ghcr.io/thin-edge/tedge:1.1.1
USER root
RUN apk update && apk add sudo
RUN echo "tedge ALL=(ALL:ALL) NOPASSWD: ALL" > /etc/sudoers.d/tedge

ARG S6_OVERLAY_VERSION=3.2.0.0

ADD https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-noarch.tar.xz /tmp
RUN tar -C / -Jxpf /tmp/s6-overlay-noarch.tar.xz
ADD https://github.com/just-containers/s6-overlay/releases/download/v${S6_OVERLAY_VERSION}/s6-overlay-x86_64.tar.xz /tmp
RUN tar -C / -Jxpf /tmp/s6-overlay-x86_64.tar.xz

COPY ./tedge /usr/bin/tedge

RUN mkdir -p /etc/s6-overlay
COPY ./s6-rc.d /etc/s6-overlay/s6-rc.d

USER tedge
ENTRYPOINT [ "/init" ]