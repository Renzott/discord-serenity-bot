#!/bin/bash

# Variables
IMAGE_NAME="docker-bot"
CONTAINER_NAME="docker-bot_container"

# Update the repository
git pull

# Get the ID of the last created image
IMAGE_ID=$(sudo docker images | grep $IMAGE_NAME | awk '{print $3}' | tail -n 1)

# Remove the last created image
sudo docker rmi $IMAGE_ID || true

# Stop the container
sudo docker stop $CONTAINER_NAME || true

# Remove the container
sudo docker rm $CONTAINER_NAME || true

# Build the image
sudo docker build -t $IMAGE_NAME .

# Run the container
sudo docker run -d --name $CONTAINER_NAME --env-file .env $IMAGE_NAME
