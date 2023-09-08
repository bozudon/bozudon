#!/usr/bin/env bash
set -xeu
set -o pipefail

[ -e .env ] && . .env || . .env.deploy

# Stop All
docker compose down

# Update repositories
git pull --recurse-submodules
git submodule update --recursive --remote

docker compose build

# Start backend only for getting client ID and secret
docker compose up -d
sleep 10

# Database initialization
echo "drop schema public cascade;" | docker compose exec -T db psql -U YOUR_USERNAME YOUR_DATABASE_NAME
echo "create schema public;" | docker compose exec -T db psql -U YOUR_USERNAME YOUR_DATABASE_NAME
cat ./backend/schema.sql | docker compose exec -T db psql -U YOUR_USERNAME YOUR_DATABASE_NAME
cat ./backend/init-data-prod.sql | sed "s@https://localhost:8080@${BOZUDON_CLIENT_URL}@g" | docker compose exec -T db psql -U YOUR_USERNAME YOUR_DATABASE_NAME

# retrieve client ID and secret
sleep 3
CLIENT_INFO=$(curl -X POST $BOZUDON_SERVER_URL/api/v1/apps -d client_name=bozudonfrontend -d redirect_uris="$BOZUDON_SERVER_URL/auth/callback" -d scopes='read write follow push')
CLIENT_ID_TMP=$(echo $CLIENT_INFO | jq -r ".client_id")
CLIENT_SECRET_TMP=$(echo $CLIENT_INFO | jq -r ".client_secret")
echo "BOZUDON_CLIENT_ID="\"$CLIENT_ID_TMP\" >> .env
echo "BOZUDON_CLIENT_SECRET="\"$CLIENT_SECRET_TMP\" >> .env

# Start All
docker compose down && docker compose up -d

echo "Default User: user1@example.com Password: password"
