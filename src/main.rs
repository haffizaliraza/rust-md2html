//! # md2htmlx
//!
//! `md2htmlx` is a lightweight and fast command-line tool written in Rust that converts Markdown (`.md`) files to HTML (`.html`) files.
//! It includes a **live watch mode**, which automatically re-renders the output HTML file every time the Markdown source is modified.
//!
//! This tool is perfect for:
//! - Developers writing documentation
//! - Bloggers writing Markdown-based content
//! - Static site generators
//!
//! ## 🔧 Features
//!
//! - ✅ Converts Markdown to HTML using `pulldown-cmark`
//! - 👀 Automatically watches the input file for changes using `notify`
//! - 🧩 Simple CLI interface powered by `clap`
//!
//! ## 🚀 Usage
//!
//! ```sh
//! md2htmlx input.md output.html
//! ```
//!
//! This will convert `input.md` to `output.html` and keep watching the file for changes.
//!
//! ### Arguments
//!
//! - `<INPUT>`: Path to the source Markdown file (e.g. `README.md`)
//! - `<OUTPUT>`: Path where the generated HTML will be saved
//!
//! ## 💡 Example
//!
//! Given a file `README.md`:
//!
//! ```md
//! # Hello World
//!
//! This is a *Markdown* example.
//! ```
//!
//! Run the tool:
//!
//! ```sh
//! md2htmlx README.md output.html
//! ```
//!
//! Output (`output.html`):
//!
//! ```html
//! <h1>Hello World</h1>
//! <p>This is a <em>Markdown</em> example.</p>
//! ```
//!
//! ## 📦 Dependencies
//!
//! - [`pulldown-cmark`](https://crates.io/crates/pulldown-cmark) for Markdown parsing
//! - [`clap`](https://crates.io/crates/clap) for command-line interface
//! - [`notify`](https://crates.io/crates/notify) for watching file changes
//!
//! ## 📄 License
//!
//! Licensed under either of:
//!
//! - MIT License ([LICENSE-MIT](https://opensource.org/licenses/MIT))
//! - Apache License, Version 2.0 ([LICENSE-APACHE](https://www.apache.org/licenses/LICENSE-2.0.html))
//!
//! ## 👤 Author
//!
//! Created by Hafiz Ali Raza (https://github.com/haffizaliraza)


use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

use clap::Parser;
use pulldown_cmark::{Parser as MdParser, Options, html};
use notify::{Watcher, RecursiveMode, recommended_watcher, EventKind};

/// Markdown to HTML converter with live watching
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Input Markdown file
    input: PathBuf,

    /// Output HTML file
    output: PathBuf,
}

fn convert_markdown_to_html(input: &PathBuf, output: &PathBuf) {
    match fs::read_to_string(input) {
        Ok(markdown_input) => {
            let mut options = Options::empty();
            options.insert(Options::ENABLE_STRIKETHROUGH);

            let parser = MdParser::new_ext(&markdown_input, options);
            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);

            if let Err(e) = fs::write(output, html_output) {
                eprintln!("❌ Failed to write to {:?}: {}", output, e);
            } else {
                println!("✅ Converted {:?} → {:?}", input, output);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to read {:?}: {}", input, e);
        }
    }
}

fn main() -> notify::Result<()> {
    let args = Cli::parse();

    convert_markdown_to_html(&args.input, &args.output);

    // File watcher
    let (tx, rx) = channel();
    let mut watcher = recommended_watcher(tx)?;

    watcher.watch(&args.input, RecursiveMode::NonRecursive)?;

    println!("👀 Watching {:?} for changes...", args.input);

    loop {
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(event) => {
                if let Ok(event) = event {
                    if matches!(event.kind, EventKind::Modify(_)) {
                        println!("🔁 File changed, re-rendering...");
                        convert_markdown_to_html(&args.input, &args.output);
                    }
                }
            }
            Err(_) => {} // Timeout, ignore and continue
        }
    }
}
