# 📝 md2html — Markdown to HTML Converter in Rust

`md2html` is a simple and efficient command-line tool written in Rust that converts Markdown (`.md`) files to HTML. It includes an optional **watch mode** that automatically re-renders the HTML file whenever the Markdown source changes.

---

## ✨ Features

- 🔁 Converts `.md` files to `.html`
- 👀 Automatically re-renders on file change (watch mode)
- ⚡ Fast and minimal — built with Rust
- 🧩 Uses `pulldown-cmark`, `clap`, and `notify` crates

---

## 📦 Installation

Make sure you have [Rust installed](https://www.rust-lang.org/tools/install).

```bash
git clone https://github.com/yourusername/md2html.git
cd md2html
cargo build --release
