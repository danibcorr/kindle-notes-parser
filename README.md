# Kindle Notes Parser

**Kindle Notes Parser** is a CLI tool written in **Rust** designed to extract and filter
notes from your Kindle books. This tool allows you to interact with your `.txt` file,
select a specific title, and generate a clean file with your annotations.

Perfect for further processing by an LLM (ChatGPT, Claude, etc.) or integration into
your personal knowledge management system.

## Installation and Compilation

Make sure you have Rust and Cargo installed.

1. **Clone the repository:**

   ```bash
   git clone https://github.com/danibcorr/kindle-notes-parser
   cd kindle-notes-parser
   ```

2. **Compile the project:**

   ```bash
   cargo build
   ```

You will find the executable at `./target/debug/kindle-notes-parser`.

## Usage

To start parsing a notes file, use the `-p` command followed by the path to your file:

```bash
./target/debug/kindle-notes-parser -p /path/to/your/annotations.txt
```

### Output

The content is automatically saved to: `outputs/[Book Name].txt`

> **Note:** This file will contain only the highlighted text, separated by line breaks,
> removing unnecessary metadata such as dates or page positions.
