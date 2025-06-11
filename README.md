# clone_grep
A Rust-based CLI tool that mimics the functionality of the classic Unix `grep` command. It efficiently searches for matching lines in large files using pattern matching, and supports options like case-insensitive search, regex support, and recursive directory scanning.

---

## 🚀 Features

- 🔍 **Fast** text pattern matching
- 📁 **Recursive** directory scanning
- 🆎 **Case-insensitive** matching
- ⚙️ Simple and clean **CLI interface**
- 🦀 Written in safe and modern **Rust**

---

## 📦 Installation & Running

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### Clone and Run

git clone https://github.com/yourusername/clone_grep.git
cd clone_grep
cargo run -- "pattern" path/to/file_or_directory
IGNORE_CASE=1 cargo run -- "pattern" path/to/file_or_directory
