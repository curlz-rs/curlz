LATEST_RELEASE_VERSION:=0.1.0-alpha.11
LATEST_RELEASE_TAG:=v${LATEST_RELEASE_VERSION}

# set docker build options used when building docker images
export DOCKER_BUILD_OPTS=--no-cache --compress

# set docker build args used when building docker images
export DOCKER_BUILD_ARGS= --build-arg CURLZ_RELEASE_TAG=${LATEST_RELEASE_TAG} \
    --build-arg CURLZ_RELEASE_VERSION=${LATEST_RELEASE_VERSION}

export DOCKER_MULTI_ARCH=linux/arm/v7,linux/amd64,linux/arm64,linux/ppc64le,linux/s390x,linux/386

login:
	echo ${GITHUB_TOKEN} | docker login ghcr.io -u sassman --password-stdin

build:
	docker build ${DOCKER_BUILD_OPTS} ${DOCKER_BUILD_ARGS} -t "ghcr.io/curlz-rs/curlz:${LATEST_RELEASE_VERSION}" -f Dockerfile .

multibuild:
	docker buildx build ${DOCKER_BUILD_OPTS} ${DOCKER_BUILD_ARGS} -t "ghcr.io/curlz-rs/curlz:${LATEST_RELEASE_VERSION}" --platform=${DOCKER_MULTI_ARCH} -f Dockerfile . --push
	docker buildx imagetools create "ghcr.io/curlz-rs/curlz:${LATEST_RELEASE_VERSION}" --tag curlz-rs/curlz:latest

publish:
	docker push ghcr.io/curlz-rs/curlz:${LATEST_RELEASE_VERSION}

test:
	docker run --rm ghcr.io/curlz-rs/curlz:${LATEST_RELEASE_VERSION} r https://httpbin.org/get
