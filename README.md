# Powergrab

Powergrab is a Rust CLI tool that displays battery information, including the current percentage and time until fully charged. This CLI tool works on both Mac and Linux.

### Motivation

I built this CLI tool to work on my MacBook in order to easily check my battery state. As a low vision user, I prefer to check my computer info using CLI tools. I could also see this being useful for anybody who frequently uses the terminal in order to check information quickly and concisely. 

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### Build and Install

1. Clone the repository:
   ```bash
   git clone https://github.com/JamisonHunter/powergrab

2. Navigate to your cloned directory and cd into powergrab.
    ```bash
    cd powergrab

3. Next build the release version with cargo.
    ```bash
    cargo build --release

4. Move the executable into your path to make it accessible via the terminal.
    ```bash
    sudo mv target/release/powergrab /usr/local/bin/

5. Lastly, type 'powergrab' into the terminal in order to check if it is working. 

### Technologies 

I chose to use Rust primarily due to its understandable file organization and easy creation of executable files. 
