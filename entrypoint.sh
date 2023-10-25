#!/bin/sh

set -e

run_backend() {
  while : ; do
    /usr/bin/gallerious
    sleep 1s
  done
}

run_backend &

set -x

exec nginx -g "daemon off;"