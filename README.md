# Winix

**Your Most Useful Linux Commands directly on Your Windows without WSL or a Linux Distro**

Winix brings familiar Unix/Linux commands to Windows natively. Currently supports `chmod`, `chown`, `uname`, and `ps` commands with more coming soon!

```
██     ██ ██ ███    ██ ██ ██   ██
██     ██ ██ ████   ██ ██  ██ ██
██  █  ██ ██ ██ ██  ██ ██   ███
██ ███ ██ ██ ██  ██ ██ ██  ██ ██
 ███ ███  ██ ██   ████ ██ ██   ██
```

## Features

- 🚀 **Native Windows** - No WSL required
- 🔧 **chmod Command** - Change file permissions with octal and symbolic notation
- 👤 **chown Command** - Change file ownership (Windows-compatible)
- 💻 **uname Command** - Display system information
- 📊 **ps Command** - List running processes with detailed information
- 🎨 **Colorized Output** - Easy-to-read terminal interface
- ⚡ **Fast & Lightweight** - Built with Rust

## Installation

### Download from GitHub Releases

1. Go to the [Releases page](https://github.com/0xsambit/winix/releases)
2. Download `winix.exe` from the latest release
3. Run it directly


## Commands

### chmod Command

Change file permissions using octal or symbolic notation.

```bash
# Octal notation
chmod 755 <"your full path for the file">
chmod 644 document.pdf

# Symbolic notation
chmod u+x script.sh
chmod a-w file.txt
chmod u=rwx,g=rx,o=r file.txt
```

### chown Command

Change file ownership (Windows-compatible implementation).

```bash
# Change owner
chown alice file.txt

# Change owner and group (Note: Group changes have limited support on Windows)
chown alice:developers file.txt

# Change only group
chown :developers file.txt
```

### uname Command

Display system information including OS, kernel version, memory, and network details.

```bash
# Show system information
uname
```

Example output:
- System name and OS version
- Kernel version  
- Host name
- Memory usage (used/total)
- Swap usage
- CPU information
- Network interfaces

### ps Command

List running processes with detailed information.

```bash
# Show running processes
ps
```

Features:
- Process ID (PID)
- Process name
- CPU usage percentage
- Memory usage
- Disk read/write statistics
- Process status
- System summary with total processes, CPU cores, and memory usage

## Available Commands

- `chmod` - Change file permissions (octal and symbolic notation)
- `chown` - Change file ownership (Windows-compatible)
- `uname` - Display system information (OS, kernel, memory, network)
- `ps` - List running processes with detailed statistics
- `help` - Show available commands
- `exit` - Exit the program

**More Unix commands coming soon!** 🚀

## Building

```bash
cargo build --release
```

## Contributing

Contributions welcome! Please open an issue or submit a pull request. Read [Contributions](https://github.com/0xsambit/winix/blob/master/CONTRIBUTING.md)

## License

MIT License
