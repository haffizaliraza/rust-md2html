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
