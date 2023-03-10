#!/bin/bash
set -e
# set -e causes the whole script to exit when a command fails, so the script can't silently fail and startup mongo.

mongosh -u ${MONGO_INITDB_ROOT_USERNAME} -p ${MONGO_INITDB_ROOT_PASSWORD} <<EOF
use Logger
db.createCollection("default")
db.createCollection("commands")

use GeneralBot
db.createCollection("config")

db.config.insertMany([
    {name: "token", config_type: "general", data: "${BOT_TOKEN}"},
    {name: "app_id", config_type: "general", data: "${APPLICATION_ID}"},
    {name: "prefix", config_type: "general", data: "${BOT_PREFIX}"},
])
EOF
