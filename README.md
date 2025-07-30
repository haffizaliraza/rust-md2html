# 📝 md2htmlx — A Simple Markdown to HTML Converter with Live Watch Mode

`md2htmlx` is a fast, lightweight, and user-friendly command-line tool written in Rust that converts Markdown files into HTML. It also supports a **live watch mode** to automatically re-render your HTML output whenever the Markdown file is edited.

Perfect for developers, bloggers, or static site generators who want quick Markdown rendering without heavy dependencies.

---

## 🚀 Features

- ✅ Convert `.md` files to `.html`
- 🔁 Automatically re-renders on file save
- ⚡ Fast and compiled with Rust
- 🧩 Uses popular, safe crates: `pulldown-cmark`, `clap`, `notify`
- 🔧 Simple CLI interface

---

## 📦 Usage
### Basic Conversion

```bash
md2htmlx input.md output.html
```

## 📦 Installation

### From crates.io:

```bash
cargo install md2htmlx
```

### Build From Source
```bash
git clone https://github.com/yourusername/md2htmlx.git
cd md2htmlx
cargo build --release
./target/release/md2htmlx input.md output.html