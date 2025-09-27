#!/bin/bash

echo "Killing any existing desktop-app processes..."
pkill -f "desktop-app" 2>/dev/null || echo "No desktop-app processes found"

# Wait a moment for processes to fully terminate
sleep 1

# Check if any processes are still using the serial port
if command -v fuser >/dev/null 2>&1; then
    echo "Checking for processes using /dev/ttyUSB0..."
    fuser /dev/ttyUSB0 2>/dev/null && echo "Warning: /dev/ttyUSB0 still in use" || echo "✓ /dev/ttyUSB0 is free"
fi

echo "Ready to run application"