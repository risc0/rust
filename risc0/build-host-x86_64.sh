#!/bin/bash

set -e
set -x

DATE=$1

. common.sh

#DOCKERS="dist-aarch64-linux dist-x86_64-linux dist-x86_64-musl"
DOCKERS="dist-x86_64-linux"

build() {
    for docker in ${DOCKERS}
    do
        (cd .. &&
             rm -rf obj &&
             src/ci/docker/run.sh ${docker})
        ${RCLONE} copy ../obj/build/dist ${CLOUD_STAGING}/${DATE}
    done
    exit 0
}

build


