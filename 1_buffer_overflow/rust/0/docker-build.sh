#!/bin/sh

PROJECT_ROOT=`git rev-parse --show-toplevel`
IMAGE_NAME="rust-security-talk"
IMAGE_VERSION="latest"
DOCKER_IMAGE=$IMAGE_NAME:$IMAGE_VERSION

if [[ "$(docker images -q $DOCKER_IMAGE 2> /dev/null)" == "" ]]; then
    docker build -f $PROJECT_ROOT/Dockerfile -t $DOCKER_IMAGE $PROJECT_ROOT
fi
docker run -t --rm --user $(id -u):$(id -g) -v `pwd`:/tmp/build:Z -w /tmp/build $DOCKER_IMAGE /bin/sh -c './build.sh'
