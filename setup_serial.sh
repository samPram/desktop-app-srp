#!/bin/bash

echo "=== Serial USB Port Setup for SRP Dyno Test ==="

# Check if user is in dialout group
if groups $USER | grep -q dialout; then
    echo "✓ User $USER is in dialout group"
else
    echo "✗ User $USER is NOT in dialout group"
    echo "  Run: sudo usermod -a -G dialout $USER"
    echo "  Then logout and login again"
    exit 1
fi

# Check for USB devices
echo ""
echo "=== Connected USB Devices ==="
lsusb | grep -E "(Silicon Labs|Arduino|CH340|FTDI|CP210)" || echo "No known Arduino/ESP32 devices found"

# Load CP210x driver
echo ""
echo "=== Loading CP210x Driver ==="
if lsmod | grep -q cp210x; then
    echo "✓ cp210x driver already loaded"
else
    echo "Loading cp210x driver..."
    sudo modprobe cp210x
    if [ $? -eq 0 ]; then
        echo "✓ cp210x driver loaded successfully"
    else
        echo "✗ Failed to load cp210x driver"
    fi
fi

# Check for serial devices
echo ""
echo "=== Serial Devices ==="
if ls /dev/ttyUSB* >/dev/null 2>&1; then
    echo "✓ Found USB serial devices:"
    ls -la /dev/ttyUSB*
elif ls /dev/ttyACM* >/dev/null 2>&1; then
    echo "✓ Found ACM serial devices:"
    ls -la /dev/ttyACM*
else
    echo "✗ No serial devices found"
    echo "  Try unplugging and reconnecting your Arduino/ESP32"
fi

# Install udev rules
echo ""
echo "=== Installing Udev Rules ==="
if [ -f "99-arduino-serial.rules" ]; then
    sudo cp 99-arduino-serial.rules /etc/udev/rules.d/
    sudo udevadm control --reload-rules
    sudo udevadm trigger
    echo "✓ Udev rules installed and reloaded"
else
    echo "✗ Udev rules file not found"
fi

echo ""
echo "=== Setup Complete ==="
echo "If your device still doesn't appear:"
echo "1. Unplug and reconnect the USB cable"
echo "2. Try a different USB port"
echo "3. Check if Arduino IDE can see the device"
echo "4. Restart the computer if needed"