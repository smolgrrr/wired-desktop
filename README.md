# nostui

[![CI](https://github.com/akiomik/nostui/workflows/CI/badge.svg)](https://github.com/akiomik/nostui/actions)

A TUI client for [Nostr](https://nostr.com)

## Getting Started

### Requirements

- git
- rust

### Installation

1. Clone this repository and move to the directory:

```shell
git clone https://github.com/akiomik/nostui
cd nostui
```

2. Install `nostui` binary:

```shell
# Install to ~/.cargo/bin/nostui
cargo install --path .
```

### Setup

1. Create a `config.json` to the following path:

- Linux: `~/.config/notsui/config.json`
- Windows: `~\AppData\Roaming\0m1\nostui`
- macOS: `~/Library/Application Support/io.0m1.nostui`

2. Add your privatekey to the `config.json`:

```jsonc
{
    "privatekey": "nsec1...",
    "relays": ["wss://nos.lol"] // optional
}
```

## Usage

### Commands

```shell
nostui
```

### Default Keybindings

| Keybinding            | Description        |
| --------------------- | ------------------ |
| `k` `up`              | Scroll up          |
| `j` `down`            | Scroll down        |
| `q` `Ctrl-c` `Ctrl-d` | Quit               |
| `home` `gg`           | Scroll to top      |
| `end` `Shift-g`       | Scroll to bottom   |
| `Ctrl-z`              | Suspend            |
| `esc`                 | Unselect           |
