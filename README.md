## Kindle Notes Parser

A Rust CLI tool that extracts and manages highlights from your Kindle's
`My Clippings.txt` file. Ideal for exporting clean notes into personal knowledge tools
(Obsidian, Notion) or as input for AI models. Licensed under [MIT](LICENSE).

### Installation

Download prebuilt binaries for Windows, macOS, and Linux from the
[releases](https://github.com/danibcorr/kindle-notes-parser/releases) page.

Alternatively, you can build from source if you have Rust installed:

```bash
git clone https://github.com/danibcorr/kindle-notes-parser
cd kindle-notes-parser
cargo build --release
```

The compiled binary will be at `target/release/knp`.

### Usage and Available Commands

On macOS/Linux, make sure the binary has execution permissions:

```bash
chmod +x knp
```

Both commands will display an interactive menu where you can select the book title you
want to operate on.

| Command                     | Description                                                                 |
| --------------------------- | --------------------------------------------------------------------------- |
| `./knp -s <INPUT>`          | Show all available book titles in the file.                                 |
| `./knp -p <INPUT> <OUTPUT>` | Extract deduplicated highlights for a selected book to the output file.     |
| `./knp -d <INPUT> <OUTPUT>` | Remove all notes for a selected book and write the rest to the output file. |
| `./knp -h`                  | Show help.                                                                  |
