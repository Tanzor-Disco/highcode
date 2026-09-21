# Highcode

CLI utility for syntax highlighting source code and copying it to the clipboard as formatted HTML.

Highcode is designed primarily for inserting highlighted source code into documents such as Microsoft Word without manually formatting it.

> **Platform:** macOS only for now.

## Features

- Syntax highlighting using [Tree-sitter](https://tree-sitter.github.io/tree-sitter/)
- Copy highlighted code to the clipboard as HTML
- Plain-text fallback for applications that do not support HTML clipboard data
- Multiple color themes
- Automatic language detection by file extension
- Simple CLI interface
- Easy to extend with additional languages

## Installation

Clone the repository and build the project:

```bash
git clone https://github.com/Tanzor-Disco/highcode.git
cd highcode
cargo build --release
```

The binary will be available at:

```text
target/release/highcode
```

## Usage

```bash
highcode <FILE_PATH>
```

For example:

```bash
highcode main.go
```

Use a different theme with:

```bash
highcode main.go --theme light
```

Available themes:

- `dark`
- `light`

Display help:

```bash
highcode --help
```

## Supported Languages

Currently supported:

- Python
- Go
- Rust
- C#

Language support is implemented using Tree-sitter grammars and `.scm` highlight queries.

## How It Works

```text
Source file
    ↓
Language detection
    ↓
Tree-sitter parser
    ↓
SCM highlight query
    ↓
Highlight elements
    ↓
HTML renderer
    ↓
macOS clipboard
    ↓
Word / other applications
```

The generated clipboard data contains both formatted HTML and plain text, allowing applications to choose the appropriate representation.

## Adding a Language

Language support is intentionally easy to extend.

When adding a new language:

1. Add the corresponding Tree-sitter grammar.
2. Add the language to the language enum.
3. Add file-extension detection.
4. Add the `.scm` highlight query.
5. Add the grammar/query mapping.

The relevant places in the code are marked with:

```text
// Add a language
```

Search for this marker to quickly find all places that need to be updated.

## Project Structure

```text
src
├── cli
│   ├── input.rs
│   ├── mod.rs
│   └── output
│       ├── macos.rs
│       └── mod.rs
├── error
│   ├── highlighter.rs
│   ├── mod.rs
│   ├── parser.rs
│   └── source.rs
├── highlighter
│   ├── highlighter.rs
│   └── mod.rs
├── main.rs
├── render
│   ├── html.rs
│   └── mod.rs
├── source
│   ├── mod.rs
│   └── read.rs
├── theme
│   ├── mod.rs
│   └── theme.rs
└── tree
    ├── finder
    │   ├── finder.rs
    │   ├── mod.rs
    │   └── queries
    │       ├── csharp.scm
    │       ├── go.scm
    │       ├── python.scm
    │       └── rust.scm
    ├── mod.rs
    └── parser
        ├── mod.rs
        └── parser.rs
```

The exact structure may change as the project develops.

## Themes

Themes define the colors used for:

- keywords
- functions
- strings
- comments
- background
- main text

Themes are implemented independently from the highlighting logic, so changing the visual style does not require changing the parser or renderer.

## Why?

Copying source code into a document usually means either losing syntax highlighting or manually formatting the code.

Highcode automates that process:

```text
source file → highcode → paste into Word
```

The project was created primarily for producing properly formatted code listings for university reports and other documents.

## Platform

Highcode currently supports **macOS**.

The clipboard integration uses macOS-specific APIs to place both HTML and plain-text representations into the system clipboard.
