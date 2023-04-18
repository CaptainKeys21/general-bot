#!/bin/bash
set -e
# set -e causes the whole script to exit when a command fails, so the script can't silently fail and startup mongo.

mongosh -u ${MONGO_INITDB_ROOT_USERNAME} -p ${MONGO_INITDB_ROOT_PASSWORD} <<EOF
rs.initiate({_id: "GenBotDev", members: [{_id: 0, host: "general-bot-database:27017"}]})

use Logger
db.createCollection("default", { capped: true, size: 5e8 })
db.createCollection("commands", { capped: true, size: 5e8 }))

use GeneralBot
db.createCollection("config")

db.config.insertMany([
    {name: "token", config_type: "general", data: "${BOT_TOKEN}"},
    {name: "app_id", config_type: "general", data: "${APPLICATION_ID}"},
    {name: "prefix", config_type: "general", data: "${BOT_PREFIX}"},
])
EOF
