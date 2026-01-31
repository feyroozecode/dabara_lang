#!/bin/bash
# Serve the Dabara web playground locally
# Usage: ./serve.sh [port]

PORT=${1:-8080}

echo "Dabara Web Playground"
echo "====================="
echo "Serving at: http://localhost:$PORT/web/"
echo "Press Ctrl+C to stop"
echo ""

python3 -m http.server "$PORT"
