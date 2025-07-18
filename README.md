# minigrep 🦀

**minigrep** is a simplified implementation of the popular Unix `grep` command, written in Rust. It allows users to search for lines containing a specific query string in a text file.

---

## 🔍 Features

- Search for a string in a text file
- Case-sensitive and case-insensitive search modes
- Built with safe, fast, and modern Rust
- Command-line interface

---

## 📦 Installation

To install and run `minigrep`, you need to have [Rust](https://www.rust-lang.org/tools/install) installed.

```bash
git clone https://github.com/your-username/minigrep.git
cd minigrep
cargo build --release
```
This create binary at ./target/release/minigrep.
Move the binary to a directory in your $PATH

```bash
sudo cp target/release/minigrep /usr/local/bin/
```

Now you can Use it as a regular command
```bash
minigrep "query" filename.txt
