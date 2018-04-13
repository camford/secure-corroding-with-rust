#!/bin/sh

PROJECT_ROOT=`git rev-parse --show-toplevel`
IMAGE_NAME="rustdev"
IMAGE_VERSION="1.19.0"
DOCKER_IMAGE=$IMAGE_NAME:$IMAGE_VERSION

if [[ "$(docker images -q $DOCKER_IMAGE 2> /dev/null)" == "" ]]; then
    docker build -f $PROJECT_ROOT/docker/$IMAGE_NAME/Dockerfile -t $DOCKER_IMAGE $PROJECT_ROOT
fi
docker run --rm --user $(id -u):$(id -g) -v `pwd`:/tmp/build:Z $DOCKER_IMAGE /bin/sh -c 'cd /tmp/build; cargo build'
