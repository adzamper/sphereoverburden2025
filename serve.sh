#!/bin/bash

##
## Local Development Server
##
## Simple script to serve the application locally for testing.
## Requires Python 3 to be installed.
##

PORT=8000

echo "Starting local development server..."
echo ""
echo "Building application..."
./build.sh

if [ $? -ne 0 ]; then
    echo "Build failed. Exiting."
    exit 1
fi

echo ""
echo "=========================================="
echo "Server running at:"
echo "  http://localhost:$PORT"
echo "=========================================="
echo ""
echo "Press Ctrl+C to stop the server"
echo ""

python3 -m http.server $PORT --directory dist
