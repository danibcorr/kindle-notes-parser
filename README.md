## Kindle Notes Parser

**Kindle Notes Parser** is a Rust-based command-line interface tool that extracts
highlights and annotations from your Kindle. Physical Kindle devices store this data in
a file called `My Clippings.txt`. This tool is ideal for users who need clean, distilled
text for personal knowledge management (like Obsidian or Notion) or as input for AI
models.

### Installation

You can download ready-to-use versions for Windows, macOS, and Linux from the
**[releases](https://github.com/danibcorr/kindle-notes-parser/releases)** page.

To build it yourself, make sure you have the Rust toolchain and Cargo installed, then
follow these steps:

1. **Clone the repository:**
   ```bash
   git clone https://github.com/danibcorr/kindle-notes-parser
   ```
2. **Compile the project:**
   ```bash
   cd kindle-notes-parser
   cargo build --release
   ```
   The compiled executable will be in `target/release/`.

### Usage

Before running the tool on macOS or Linux, you must grant it execution permissions
_(replace `knp` with the name of the file you downloaded)_:

```bash
chmod +x knp
```

To view all available options and features, use the `help` flag:

```bash
./knp -h
```
