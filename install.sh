#!/bin/bash

set -e

# Function to check if a command exists
command_exists() {
  command -v "$1" >/dev/null 2>&1
}

# Install Rust if not already installed
if ! command_exists cargo; then
  echo "Rust is not installed. Installing Rust..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source $HOME/.cargo/env
else
  echo "Rust is already installed."
fi

# Compile the Rust script
echo "Compiling the Rust script..."
cd rust_script
cargo build --release
cd ..

# Move the executable to /usr/local/bin
echo "Moving the executable to /usr/local/bin..."
sudo mv rust_script/target/release/weather_info /usr/local/bin/weather_info

# Add the custom prompt to .zshrc
echo "Adding custom prompt to .zshrc..."
if ! grep -q "source ~/weather-prompt/zsh/weather.zsh" ~/.zshrc; then
  echo "source ~/weather-prompt/zsh/weather.zsh" >> ~/.zshrc
else
  echo "Custom prompt is already added to .zshrc."
fi

echo "Installation complete! Please restart your terminal."