#!/usr/bin/sh

# Build a development image.
docker build \
	-f docker/dev.Dockerfile \
	-t bumper-dev:latest \
	.

# Start the container.
docker run \
	--name bumper-dev \
	-d \
	--rm \
	-v `pwd`:/app \
	-p 8000:8000 \
	bumper-dev:latest
