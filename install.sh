#!/bin/bash

# Print colored output
print_colored() {
  GREEN='\033[0;32m'
  BLUE='\033[0;34m'
  RED='\033[0;31m'
  NC='\033[0m' # No Color

  echo -e "${!1}${2}${NC}"
}

# Check if command exists
command_exists() {
  command -v "$1" >/dev/null 2>&1
}

# Check OS
check_os() {
  if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "linux"
  elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "macos"
  else
    echo "unsupported"
  fi
}

# Install system dependencies
install_system_dependencies() {
  OS=$(check_os)
  if [ "$OS" = "linux" ]; then
    print_colored "BLUE" "Installing system dependencies for Linux..."
    sudo apt-get update
    sudo apt-get install -y build-essential curl wget git pkg-config
  elif [ "$OS" = "macos" ]; then
    print_colored "BLUE" "Installing system dependencies for macOS..."
    if ! command_exists brew; then
      /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    fi
    brew install curl wget git pkg-config
  else
    print_colored "RED" "Unsupported operating system"
    exit 1
  fi
}

# Install Rust and Cargo
install_rust() {
  if command_exists rustc && command_exists cargo; then
    print_colored "GREEN" "Rust and Cargo are already installed"
    print_colored "BLUE" "Updating Rust..."
    rustup update
  else
    print_colored "BLUE" "Installing Rust and Cargo..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
  fi
}

# Install noirlings binary
install_noirlings() {
  print_colored "BLUE" "Installing noirlings..."
  cargo install --path .
  if command_exists noirlings; then
    print_colored "GREEN" "noirlings installed successfully"
  else
    print_colored "RED" "Failed to install noirlings"
    exit 1
  fi
}

# Install Nargo
install_nargo() {
  if command_exists nargo; then
    print_colored "GREEN" "Nargo is already installed"
    print_colored "BLUE" "Updating Nargo..."
    noirup
  else
    print_colored "BLUE" "Installing Nargo..."
    curl -L https://raw.githubusercontent.com/noir-lang/noirup/refs/heads/main/install | bash
    source "$HOME/.nargo/env"
    noirup
  fi
}

# Main installation process
main() {
  print_colored "BLUE" "Starting installation process..."

  # Check and install system dependencies
  install_system_dependencies

  # Install Rust and Cargo
  install_rust

  # Install Nargo
  install_nargo

  # Install noirlings
  install_noirlings

  # Final verification
  print_colored "BLUE" "Verifying installations..."

  # Verify Rust installation
  if command_exists rustc && command_exists cargo; then
    print_colored "GREEN" "✓ Rust and Cargo are installed:"
    rustc --version
    cargo --version
  else
    print_colored "RED" "✗ Rust installation failed"
    exit 1
  fi

  # Verify Nargo installation
  if command_exists nargo; then
    print_colored "GREEN" "✓ Nargo is installed:"
    nargo --version
  else
    print_colored "RED" "✗ Nargo installation failed"
    exit 1
  fi

  # Verify noirlings installation
  if command_exists noirlings; then
    print_colored "GREEN" "✓ noirlings is installed:"
    noirlings --version
  else
    print_colored "RED" "✗ noirlings installation failed"
    exit 1
  fi

  print_colored "GREEN" "Installation completed successfully!"
  print_colored "BLUE" "Please run 'source $HOME/.cargo/env' and 'source $HOME/.nargo/env' or restart your terminal to use the installed tools."
}

# Run main installation
main
