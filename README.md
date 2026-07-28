# SSH Rover

A terminal user interface (TUI) for discovering hosts on a local network and connecting to them over SSH.

## Scan Current Network

![Scan Current Network](assets/screenshot.scan_current_network.png)

## Manual Connect

![Manual Connect](assets/screenshot.manual_connect.png)

## Features

- Discover hosts on your local network
- Scan custom IPv4 CIDR ranges
- Connect to a single host over SSH
- Shows the correct install command for Nmap and OpenSSH based on your Linux distro
- Multiple built-in color themes
- Built with Ratatui and Crossterm

## Requirements

- OpenSSH
- Nmap

## Installation

```bash
cargo install ssh-rover
```

## Usage

```bash
ssh-rover
```

## License

MIT
