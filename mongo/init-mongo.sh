#!/bin/bash
set -e
# set -e causes the whole script to exit when a command fails, so the script can't silently fail and startup mongo.

mongo -u ${MONGO_INITDB_ROOT_USERNAME} -p ${MONGO_INITDB_ROOT_PASSWORD} <<EOF
use logger
db.createCollection("default")
EOF
