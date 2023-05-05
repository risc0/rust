#!/bin/bash

case "$DATE" in
    ????-??-??) ;;
    *)
        echo "Usage: $0 YYYY-MM-DD" 2>&1
        exit 1
        ;;
esac

CLOUD_DIST="gcp:risc0-rust-dist/dist"
CLOUD_STAGING="gcp:risc0-rust-dist/stage"

# display progress
RCLONE="rclone -P"
