# Powergrab

Powergrab is a Rust CLI tool that displays battery information, including the current percentage and time until fully charged.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### Build and Install

1. Clone the repository:
   ```bash
   git clone https://github.com/JamisonHunter/powergrab

2. Navigate to your cloned directory and cd into powergrab.
    '''bash
    cd powergrab

3. Next build the release version with cargo.
    '''bash
    cargo build --release

4. Move the executable into your path to make it accessible via the terminal.
    '''bash
    sudo mv target/release/powergrab /usr/local/bin/

