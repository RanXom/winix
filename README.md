# Winix

<div align="center">

![Platform](https://img.shields.io/badge/platform-Windows-blue)
![Language](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-green)
![Release](https://img.shields.io/github/v/release/0xsambit/winix)
![Downloads](https://img.shields.io/github/downloads/0xsambit/winix/total)

**Native Unix Command Implementation for Windows**

A high-performance command-line utility that brings essential Unix/Linux functionality to Windows environments without requiring WSL or virtualization.

```
██     ██ ██ ███    ██ ██ ██   ██
██     ██ ██ ████   ██ ██  ██ ██
██  █  ██ ██ ██ ██  ██ ██   ███
██ ███ ██ ██ ██  ██ ██ ██  ██ ██
 ███ ███  ██ ██   ████ ██ ██   ██
```

</div>


![Screenshot (750)](https://github.com/user-attachments/assets/47c994fc-0937-4840-af18-61b702da76e8)

![Screenshot (754)](https://github.com/user-attachments/assets/7447b0df-1f0a-4d1b-af85-daee694d5341)

## Overview

Winix is a cross-platform command-line application designed to bridge the gap between Unix/Linux and Windows environments. Built with Rust for optimal performance and reliability, it provides native implementations of essential Unix commands that Windows users frequently need.

## Key Features

### **Native Windows Integration**

- Direct Windows API integration without virtualization overhead
- No dependency on Windows Subsystem for Linux (WSL)
- Seamless integration with existing Windows workflows

### **High Performance Architecture**

- Written in Rust for memory safety and zero-cost abstractions
- Optimized for low resource consumption
- Fast startup and execution times

### **Enhanced User Experience**

- Colorized terminal output for improved readability
- Consistent command-line interface across all utilities
- Windows-compatible file path handling

### **Comprehensive Command Suite**

- File permission management (`chmod`)
- Ownership control (`chown`)
- System information retrieval (`uname`)
- Process monitoring (`ps`)
- Extensible architecture for additional commands

## Installation

### Binary Release

Download the latest release from the [GitHub Releases](https://github.com/0xsambit/winix/releases) page:

```powershell
# Download and extract the latest release
curl -L -o winix.exe https://github.com/0xsambit/winix/releases/latest/download/winix.exe
```

### Build from Source

```powershell
git clone https://github.com/0xsambit/winix.git
cd winix
cargo build --release
```

## Project Structure

The project follows a modular architecture with each command implemented as a separate module:

```
src/
├── main.rs         # Application entry point and CLI interface
├── chmod.rs        # File permission management
├── chown.rs        # File ownership operations
├── uname.rs        # System information utilities
├── ps.rs           # Process management tools
└── ...             # Additional command modules
```

## Development

### Prerequisites

- Rust 1.70+
- Windows 10+ or Windows Server 2019+

### Testing

```powershell
cargo test
cargo test --release
```

### Contributing

We welcome contributions to expand Winix's functionality. Please refer to our [Contributing Guidelines](CONTRIBUTING.md) for detailed information on:

- Code style and formatting standards
- Pull request submission process
- Issue reporting guidelines

### Roadmap

Future development plans include:

- **Extended Command Set**: Implementation of additional Unix utilities
- **Configuration Management**: User-customizable command behavior
- **Plugin Architecture**: Support for third-party command extensions
- **Cross-Platform Support**: Expansion to Linux and macOS environments

## Technical Specifications

| Component                   | Technology              |
| --------------------------- | ----------------------- |
| **Core Language**           | Rust 2021 Edition       |
| **Minimum Windows Version** | Windows 10 (1903+)      |
| **Architecture Support**    | x86_64                  |
| **Dependencies**            | Minimal external crates |
| **Binary Size**             | < 5MB                   |

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for complete details.

---

<div align="center">

**Built with ❤️ using Rust**

[Report Bug](https://github.com/0xsambit/winix/issues) • [Request Feature](https://github.com/0xsambit/winix/issues) • [Documentation](https://github.com/0xsambit/winix/wiki)

</div>
