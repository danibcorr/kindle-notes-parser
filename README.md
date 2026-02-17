# Kindle Notes Parser

Kindle Notes Parser is a command-line interface utility developed in Rust for the
purpose of extracting annotations from Kindle devices.

This tool is designed for users who require distilled text for integration into personal
knowledge management systems or as optimized input for large language models.

## Installation and Compilation

Pre-compiled binaries for Linux, Windows, and macOS are available for immediate use on
the official releases page. Users who prefer to build the software from the source must
ensure that the Rust toolchain and Cargo package manager are installed on their system.

To begin the build process, clone the repository using the following command:

```bash
git clone https://github.com/danibcorr/kindle-notes-parser
```

Navigate into the project directory and execute the compilation command:

```bash
cargo build --release
```

The resulting executable is located within the `target/release/` directory.

## Usage

The application requires a path to the source clippings file provided via the `-p` flag.
Execution follows this syntax:

```bash
./target/release/kindle-notes-parser -p /path/to/your/annotations.txt
```

The parser analyzes the provided file and allows for the selection of specific titles.
Upon completion, the tool generates a clean text file containing only the highlighted
passages.

Processed content is automatically exported to the `outputs/` directory using the naming
convention `[Book Name].txt`. The output logic prioritizes the integrity of the original
highlights while systematically removing non-essential data such as timestamps, page
numbers, and location coordinates.
