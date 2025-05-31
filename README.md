# Winix

**Your Most Useful Linux Commands directly on Your Windows without WSL or a Linux Distro**

Winix brings familiar Unix/Linux commands to Windows natively. Currently supports `chmod` with more commands coming soon!

```
██     ██ ██ ███    ██ ██ ██   ██
██     ██ ██ ████   ██ ██  ██ ██
██  █  ██ ██ ██ ██  ██ ██   ███
██ ███ ██ ██ ██  ██ ██ ██  ██ ██
 ███ ███  ██ ██   ████ ██ ██   ██
```

## Features

- 🚀 **Native Windows** - No WSL required
- 🔧 **chmod Command** - First of many Unix commands (more coming!)
- 🎨 **Colorized Output** - Easy-to-read terminal interface
- ⚡ **Fast & Lightweight** - Built with Rust

## Installation

### Download from GitHub Releases

1. Go to the [Releases page](https://github.com/0xsambit/winix/releases)
2. Download `winix.exe` from the latest release
3. Run it directly


### chmod Command

```bash
# Octal notation
chmod 755 <"your full path for the file">
chmod 644 document.pdf

# Symbolic notation
chmod u+x script.sh
chmod a-w file.txt
chmod u=rwx,g=rx,o=r file.txt
```

## Available Commands

- `chmod` - Change file permissions (octal and symbolic notation)
- `help` - Show available commands
- `exit` - Exit the program

**More Unix commands coming soon!** 🚀

## Building

```bash
cargo build --release
```

## Contributing

Contributions welcome! Please open an issue or submit a pull request.

## License

MIT License
