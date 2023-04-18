docker compose up -d db

if [ -z "$1" ]; then
    docker compose up -d dev-bot
    docker compose up -d dev-api
    docker compose up -d dev-dashboard
    exit 0
fi

if [ $1 = "bot" ]; then
    docker compose up dev-bot
    exit 0
fi

if [ $1 = "api" ]; then
    docker compose up dev-api
    exit 0
fi

if [ $1 = "dashboard" ]; then
    docker compose up dev-dashboard
    exit 0
fi