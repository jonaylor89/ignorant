
# Ignorant

[![Crates.io](https://img.shields.io/crates/v/ignorant.svg)](https://crates.io/crates/ignorant)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

> 🔍 **A fast, reliable phone number OSINT tool**

Ignorant allows you to check if a phone number is associated with accounts on various platforms like Amazon, Instagram, and Snapchat. This is a complete Rust port of the original Python tool, offering better performance, memory safety, and zero runtime dependencies.

**⚠️ Important**: This tool does not alert the target phone number and is designed for defensive security purposes and OSINT research.

![Demo](https://github.com/megadose/gif-demo/raw/master/ignorant-demo.gif)

### From Crates.io
```bash
cargo install ignorant-rs
```

### From Source
```bash
git clone https://github.com/jonaylor89/ignorant.git
cd ignorant/
cargo build --release
```

## 📚 Usage

### Basic Usage
```bash
ignorant 33 644637111
```

### Command Line Options
```bash
ignorant [OPTIONS] <COUNTRY_CODE> <PHONE>

Arguments:
  <COUNTRY_CODE>  Country code of the phone (Example: 33)
  <PHONE>         Target phone number (Example: 644637111)

Options:
      --only-used          Display only sites where the phone number is used
      --no-color           Disable colored terminal output
      --no-clear           Don't clear the terminal before showing results
  -T, --timeout <TIMEOUT> Set request timeout in seconds [default: 10]
  -h, --help              Print help information
  -V, --version           Print version information
```

### Examples
```bash
# Basic check
ignorant 1 5551234567

# Only show platforms where the number exists
ignorant 44 7700900000 --only-used

# Disable colors and clearing for logging
ignorant 33 612345678 --no-color --no-clear

# Set custom timeout
ignorant 49 1234567890 --timeout 30
```

## 📤 Output Format

The tool outputs results in a clear, color-coded format:
- 🟢 **[+]** Phone number found on platform
- 🟣 **[-]** Phone number not found on platform
- 🔴 **[x]** Rate limited or error occurred

Each result includes:
```json
{
  "name": "instagram",
  "domain": "instagram.com",
  "method": "other",
  "frequent_rate_limit": false,
  "rate_limit": false,
  "exists": false
}
```

### Running Tests
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific module tests
cargo test modules::amazon

# Run integration tests
cargo test --test integration_tests
```

### Building
```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release
```

## 🛡️ Rate Limiting

If you encounter rate limits:
- Use a VPN or proxy to change your IP address
- Increase the timeout with `--timeout`
- Wait between requests
- Consider using different user agents (tool rotates automatically)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request


## 🙏 Acknowledgments

- [yazeed44](https://github.com/yazeed44) - Original contributor
- Original Python implementation contributors
- Rust community for excellent tooling and documentation

## ⚖️ Legal Disclaimer

This tool is provided for educational and research purposes only. Users are responsible for complying with applicable laws and terms of service of the platforms being checked. The authors are not responsible for any misuse of this tool.

## 📝 License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

---

**Made with 🦀 Rust** | **Original Python version by [megadose](https://github.com/megadose)**
