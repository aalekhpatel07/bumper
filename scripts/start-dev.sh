#!/usr/bin/sh

echo "Starting the container."
docker run \
	--name bumper-dev \
	-d \
	--rm \
	-v `pwd`:/app \
	-p 8000:8000 \
	aalekhpatel07/rust:1.0 \
	/bin/sleep infinity

echo "Starting the server."
docker exec \
	-d \
	bumper-dev \
	python3 -m http.server --bind 0.0.0.0 8000 --directory /app/public

echo "Starting the src watcher."
docker exec \
	-d \
	bumper-dev \
	run-when -f /app/src --command-file /app/scripts/build.sh --recursive -t 800ms
