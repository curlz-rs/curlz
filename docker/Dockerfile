FROM curlimages/curl:8.00.1 as builder

LABEL org.opencontainers.image.source https://github.com/curlz-rs/curlz
ARG CURLZ_RELEASE_TAG=v0.1.0-alpha.11
ARG CURLZ_GIT_REPO=https://github.com/curlz-rs/curlz
ARG LABEL_VERSION=1.0.0
ARG LABEL_NAME=curlz
ARG LABEL_DESC=curlz

WORKDIR /tmp
RUN curl ${CURLZ_GIT_REPO}/releases/download/${CURLZ_RELEASE_TAG}/curlz-${CURLZ_RELEASE_TAG}-x86_64-unknown-linux-musl.tar.gz \
    -L -o /tmp/curlz.tar.gz && \
    tar xzfv /tmp/curlz.tar.gz curlz && \
    chmod a+x /tmp/curlz

FROM curlimages/curl:8.00.1

COPY --from=builder "/tmp/curlz" "/usr/bin/curlz"
USER curl_user

COPY "entrypoint.sh" "/entrypoint.sh"
CMD ["curlz"]
ENTRYPOINT ["/entrypoint.sh"]
