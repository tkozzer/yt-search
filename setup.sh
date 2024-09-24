#!/bin/bash

# Check if Homebrew is installed
if ! command -v brew &> /dev/null
then
    echo "Homebrew is not installed. Installing Homebrew..."
    /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
else
    echo "Homebrew is already installed."
fi

# Install Tor
if ! command -v tor &> /dev/null
then
    echo "Installing Tor..."
    brew install tor
else
    echo "Tor is already installed."
fi

# Start Tor
echo "Starting Tor..."
brew services start tor

echo "Tor SOCKS proxy is now running on 127.0.0.1:9050"
echo "You can use this proxy with the YouTube Search Scraping Library"