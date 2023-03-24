docker compose up -d db

if [ -z "$1" ]; then
    docker compose up dev-bot
    docker compose up dev-api
fi

if [ $1 = "bot" ]; then
    docker compose up dev-bot
fi

if [ $1 = "api" ]; then
    docker compose up dev-api
fi