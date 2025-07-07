# RustHound: Real-Time Log Analysis & Monitoring

![RustHound Banner](https://your-image-url.com/banner.png) <!-- Replace with a real banner URL if you have one -->

**RustHound is a powerful, open-source, real-time log analysis and monitoring tool crafted with Rust.** It's designed for system administrators, developers, and anyone who needs to keep a close eye on log data.

Say goodbye to manually hunting for critical events, patterns, and anomalies in your log files. With RustHound, you can automatically monitor your logs against a defined set of rules and get valuable insights instantly!

## Table of Contents

- [Features](#-features)
- [Installation](#-installation)
- [Usage](#-usage)
- [Configuration](#-configuration)
- [Contributing](#-contributing)
- [License](#-license)

## ✨ Features

*   **Real-Time Log Monitoring:** Tails specified log files in real-time, processing new entries as they appear.
*   **Rule-Based Pattern Matching:** Match specific text patterns or regex in log entries using configurable rules (`rules.toml`).
*   **Frequency Analysis:** Track how often defined patterns occur within specific time windows and get alerted when thresholds are breached.
*   **Flexible Output Options:** Print analysis results to the console or save them to structured JSON files.
*   **Cross-Platform Compatibility:** Works seamlessly on Linux, macOS, and Windows.

## 🚀 Installation

### Prerequisites

Ensure you have **Rust** installed. You can install it via `rustup` from [https://rustup.rs/](https://rustup.rs/).

### From Crates.io (Recommended)

*Coming soon! Once published, you'll be able to install with:*
```bash
cargo install rust_hound
```

### From Source

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/your-username/RustHound.git # Update with your repo URL
    cd RustHound
    ```

2.  **Install using `cargo`:**
    This command will build the binary and place it in your cargo bin path (`~/.cargo/bin/`), making it available system-wide.
    ```bash
    cargo install --path .
    ```

## USAGE

Once installed, you can use the `rust_hound` command directly from your terminal.

### Basic Usage

```bash
# Analyze a single file
rust_hound --file /path/to/your/logfile.log

# Analyze all .log files in a directory
rust_hound --dir /path/to/your/logs/
```

### Command-Line Arguments

| Flag | Alias | Description | Default |
| :--- | :--- | :--- | :--- |
| `--file <FILE>` | `-f` | Path to a single log file. | `sample.log` |
| `--dir <DIR>` | `-d` | Path to a directory containing .log files. | `.` |
| `--rules <RULES>` | `-r` | Path to the rules file. | `rules.toml` |
| `--output <OUTPUT>` | `-o` | Output format: `console`, `json`, `both`. | `console` |
| `--follow` | `-F` | Enable tail -f mode (real-time monitoring). | `false` |
| `--severity <SEVERITY>` | `-s` | Filter by severity: `critical`, `high`, etc. | |
| `--verbose` | `-v` | Enable debug output. | `false` |
| `--help` | `-h` | Print help information. | |
| `--version` | `-V` | Print version information. | |

**Example:** Monitor a directory in real-time and output to JSON:
```bash
rust_hound --dir /var/log/ --follow --output json
```

## ⚙️ Configuration

RustHound's behavior can be customized via the `rules.toml` file. By default, it looks for this file in the current directory. You can specify a different path using the `--rules` argument.

Key configuration options include:
*   `error_patterns` & `warning_patterns`: Simple string matches for common error/warning keywords.
*   `regex_rules`: Define complex patterns with severity levels (e.g., `critical`, `high`).
*   `frequency_rules`: Set thresholds for how many times an event can occur in a given time window.

## 🤝 Contributing

Contributions are welcome! If you're passionate about Rust, log analysis, or systems monitoring, feel free to fork the repository, open issues, or submit pull requests.

## 📜 License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE) file for details.
