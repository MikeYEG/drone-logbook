#!/bin/sh
set -e

# Start the Axum backend in the background
echo "Starting Drone Logbook API server on port 3001..."
/app/drone-logbook &

# Start nginx in the foreground
echo "Starting nginx on port 80..."
exec nginx -g 'daemon off;'
