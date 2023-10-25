#!/bin/sh

set -e

run_backend() {
  while : ; do
    BOT_TOKEN=$BOT_TOKEN FORUM_ID=$FORUM_ID GUILD_ID=$GUILD_ID /usr/bin/gallerious
    sleep 1s
  done
}

run_backend &

set -x

exec nginx -g "daemon off;"