# Winix

**Your Most Useful Linux Commands directly on Your Windows without WSL or a Linux Distro**

Winix is a Rust-powered command-line tool that brings familiar Unix/Linux file permission commands to Windows. It provides a native Windows implementation of the `chmod` command, working directly with Windows Access Control Lists (ACLs) to manage file permissions.

```
██     ██ ██ ███    ██ ██ ██   ██
██     ██ ██ ████   ██ ██  ██ ██
██  █  ██ ██ ██ ██  ██ ██   ███
██ ███ ██ ██ ██  ██ ██ ██  ██ ██
 ███ ███  ██ ██   ████ ██ ██   ██
```

## Features

- 🚀 **Native Windows Implementation** - No WSL or Linux subsystem required
- 🔒 **Full chmod Support** - Both octal and symbolic notation
- 🎨 **Colorized Output** - Easy-to-read terminal interface
- ⚡ **Fast Performance** - Built with Rust for optimal speed
- 🛡️ **Windows ACL Integration** - Works directly with Windows security model

## Installation

### From Source

1. Clone the repository:

```bash
git clone <repository-url>
cd winix
```

2. Build the project:

```bash
cargo build --release
```

3. The executable will be available at `target/release/winix.exe`

### From Release

Download the latest release from the releases page and extract `winix.exe` to a directory in your PATH.

## Usage

### Interactive Mode

Run `winix` without arguments to enter interactive mode:

```bash
winix
```

This will start the Winix shell where you can run commands interactively.

### Available Commands

#### chmod

Change file permissions using either octal or symbolic notation.

**Octal Mode:**

```bash
chmod 755 myfile.txt
chmod 644 document.pdf
chmod 777 script.sh
```

**Symbolic Mode:**

```bash
chmod u+x script.sh              # Add execute permission for owner
chmod g-w,o-w file.txt          # Remove write permission for group and others
chmod a=r file.txt              # Set read-only for all
chmod u=rwx,g=rx,o=r file.txt   # Detailed permission setting
chmod +x script.sh              # Add execute for all (equivalent to a+x)
```

**Supported Permission Characters:**

- `r` - Read permission
- `w` - Write permission
- `x` - Execute permission
- `X` - Execute permission (only for directories or files with existing execute permission)

**Supported User Categories:**

- `u` - Owner (user)
- `g` - Group
- `o` - Others
- `a` - All (owner, group, and others)

**Supported Operations:**

- `+` - Add permissions
- `-` - Remove permissions
- `=` - Set exact permissions

#### Other Commands

- `help` - Show available commands
- `exit` or `quit` - Exit the program

## Examples

### Basic Usage

```bash
# Make a script executable
chmod +x myscript.sh

# Set file to read-only for all users
chmod a=r important_file.txt

# Give full permissions to owner, read/execute to group, read-only to others
chmod 754 myprogram.exe

# Remove write permissions for group and others
chmod go-w sensitive_file.txt
```

### Advanced Usage

```bash
# Multiple permission changes in one command
chmod u+rwx,g+rx,o+r newfile.txt

# Set different permissions for multiple files
chmod 644 *.txt
chmod 755 *.exe
```

## Technical Details

### Windows ACL Integration

Winix translates Unix-style permissions to Windows Access Control Lists (ACLs):

- **Owner permissions** → Current user SID + Administrators group
- **Group permissions** → Users group (S-1-5-32-545)
- **Other permissions** → Everyone group (S-1-1-0)

### Permission Mapping

| Octal | Binary | Permissions | Description       |
| ----- | ------ | ----------- | ----------------- |
| 0     | 000    | ---         | No permissions    |
| 1     | 001    | --x         | Execute only      |
| 2     | 010    | -w-         | Write only        |
| 3     | 011    | -wx         | Write and execute |
| 4     | 100    | r--         | Read only         |
| 5     | 101    | r-x         | Read and execute  |
| 6     | 110    | rw-         | Read and write    |
| 7     | 111    | rwx         | Full permissions  |

## Dependencies

- **clap** - Command-line argument parsing
- **colored** - Terminal color output
- **winapi** - Windows API bindings
- **windows-acl** - Windows ACL manipulation

## Building

### Requirements

- Rust 2024 edition or later
- Windows development environment

### Build Commands

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Run the application
cargo run
```

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes
4. Add tests if applicable
5. Commit your changes: `git commit -am 'Add feature'`
6. Push to the branch: `git push origin feature-name`
7. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Roadmap

- [ ] Add more Unix commands (ls, cp, mv, etc.)
- [ ] Support for recursive permission changes (-R flag)
- [ ] Configuration file support
- [ ] Windows PowerShell module integration
- [ ] GUI version

## Troubleshooting

### Common Issues

**Permission Denied Errors:**

- Run as Administrator for system files
- Ensure you have appropriate permissions on the target files

**File Not Found:**

- Check file paths and ensure files exist
- Use quotes around filenames with spaces

**Invalid Permissions:**

- Verify octal values are between 0-7
- Check symbolic notation syntax

## Support

If you encounter any issues or have questions:

1. Check the [troubleshooting](#troubleshooting) section
2. Search existing issues on GitHub
3. Create a new issue with detailed information about your problem

---

**Made with ❤️ in Rust** | Bringing Unix power to Windows users
