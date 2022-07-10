#!/usr/bin/sh

echo "Starting the watcher."
run-when -f /app/bumper-web --command-file /app/scripts/build.sh --recursive -t 1000ms &

echo "Starting the server."
python3 -m http.server --bind 0.0.0.0 8000 --directory /app/public 

