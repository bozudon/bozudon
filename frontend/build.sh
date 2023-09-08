#!/bin/bash

# Dockerイメージをビルドする
docker build -t myapp:1.0 .

# Dockerイメージを実行する（バックグラウンドで実行（'-d' オプション））
docker run -d -p 3000:3000 --name my_app_container myapp:1.0

# Dockerコンテナを停止するコマンド
# docker stop my_app_container

# Dockerコンテナに入るコマンド
# docker exec -it my_app_container /bin/sh
