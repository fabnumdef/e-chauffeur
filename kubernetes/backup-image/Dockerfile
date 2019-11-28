FROM rustlang/rust:nightly-alpine

RUN apk update \
    && apk upgrade \
    && apk add jq curl
WORKDIR /root
COPY date-filter ./date-filter
RUN cd ./date-filter \
    && cargo build --locked --release \
    && mv target/release/date-filter /usr/bin/ \
    && cd .. \
    && rm -rf date-filter

ENTRYPOINT ["/bin/sh"]