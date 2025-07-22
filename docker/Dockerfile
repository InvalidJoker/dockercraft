FROM eclipse-temurin:21-jre-alpine

LABEL author="InvalidJoker"

ARG IN_DEPLOYMENT=true
ARG DOWNLOAD_URL="https://piston-data.mojang.com/v1/objects/6bce4ef400e4efaa63a13d5e6f6b500be969ef81/server.jar"

RUN apk add --no-cache \
    curl \
    lsof \
    ca-certificates \
    openssl \
    git \
    tar \
    sqlite \
    fontconfig \
    tzdata \
    iproute2 \
    freetype \
    tini \
    zip \
    unzip && \
    adduser -D -h /app container

WORKDIR /app
COPY --chown=container:container build-config/ ./

RUN curl -L -o server.jar "${DOWNLOAD_URL}" && \
    if [ ! -f server.jar ]; then echo "Failed to download server jar from ${DOWNLOAD_URL}"; exit 1; fi && \
    echo "Downloaded server.jar successfully." && \
    chown container:container server.jar && \
    chmod 777 server.jar && \
    echo "eula=true" > eula.txt && \
    mkdir -p logs && touch logs/latest.log && \
    (java -jar server.jar --nogui & echo $! > /tmp/server.pid) && \
    tail -f logs/latest.log | grep -m 1 ")! For help, type" && \
    kill -9 $(cat /tmp/server.pid) || true && \
    rm -f server.properties && \
    chown -R container:container /app && chmod -R 777 /app

# remove the build config files
RUN rm -rf /app/bukkit.yml /app/server.properties

USER container
ENV USER=container HOME=/app
WORKDIR /app

STOPSIGNAL SIGINT

COPY --chown=container:container ./entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/sbin/tini", "-g", "--"]
CMD ["/entrypoint.sh"]
