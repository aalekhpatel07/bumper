#!/usr/bin/sh

echo "Starting the watcher."
run-when -f /app/src --command-file /app/scripts/build.sh --recursive -t 800ms &

echo "Starting the server."
python3 -m http.server --bind 0.0.0.0 8000 --directory /app/public 

