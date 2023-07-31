#!/bin/bash
docker compose up -d db

if [ -z "$1" ]; then
    docker compose up -d dev-bot
    docker compose up -d dev-api
    docker compose up -d dev-dashboard
    exit 0
fi

if [[ "$@" == *"db-only"* ]]; then
    exit 0
fi

if [[ "$@" == *"bot"* ]]; then
    docker compose up -d dev-bot
fi

if [[ "$@" == *"api"* ]]; then
    docker compose up -d dev-api
fi

if [[ "$@" == *"dashboard"* ]]; then
    docker compose up -d dev-dashboard
fi

exit 0