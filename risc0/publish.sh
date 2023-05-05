#!/bin/bash

set -e
set -x

DATE=$1
. common.sh

publish() {
    rm -rf staging manifests
    ${RCLONE} copy ${CLOUD_STAGING}/${DATE} staging
    mkdir manifests
    cargo run --manifest-path ../src/tools/build-manifest/Cargo.toml staging manifests ${DATE} https://storage.googleapis.com/risc0-rust-dist/dist nightly
    (cd manifests &&
         for i in *.toml
         do
             sha256sum $i | awk '{print $1}' > $i.sha256
         done
    )
    ${RCLONE} copy staging ${CLOUD_DIST}/${DATE}
    ${RCLONE} copy manifests ${CLOUD_DIST}/${DATE}
    ${RCLONE} copy manifests ${CLOUD_DIST}
    exit 0
}

publish

