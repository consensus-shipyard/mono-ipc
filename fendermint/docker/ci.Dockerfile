FROM --platform=$BUILDPLATFORM ubuntu:jammy

RUN apt-get update && \
  apt-get install -y libssl3 ca-certificates curl && \
  rm -rf /var/lib/apt/lists/*

ENV FM_HOME_DIR=/fendermint
ENV HOME=$FM_HOME_DIR
WORKDIR $FM_HOME_DIR

EXPOSE 26658
EXPOSE 8445
EXPOSE 9184

ENTRYPOINT ["docker-entry.sh"]
CMD ["run"]

STOPSIGNAL SIGTERM

ENV FM_ABCI__LISTEN__HOST=0.0.0.0
ENV FM_ETH__LISTEN__HOST=0.0.0.0
ENV FM_METRICS__LISTEN__HOST=0.0.0.0

RUN mkdir /fendermint/logs
RUN chmod 777 /fendermint/logs

COPY fendermint/builtin-actors/output/bundle.car $FM_HOME_DIR/bundle.car
COPY contracts/out $FM_HOME_DIR/contracts
COPY fendermint/docker/docker-entry.sh /usr/local/bin/docker-entry.sh
COPY fendermint/actors/output/custom_actors_bundle.car  $FM_HOME_DIR/custom_actors_bundle.car
COPY fendermint/app/config $FM_HOME_DIR/config
COPY binary/fendermint /usr/local/bin/fendermint
COPY binary/ipc-cli /usr/local/bin/ipc-cli