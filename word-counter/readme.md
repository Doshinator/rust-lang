# Word Counter CLI

A command-line tool written in Rust that counts lines, words, and characters in a text file, and identifies the most frequent words.

---

## Features

- Count **lines**, **words**, and **characters** in a text file.
- Display the **top N most frequent words**.
- Handles **punctuation** and **case-insensitive word counting**.
- Graceful error handling for missing files or invalid input.

---

## Installation

Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.

Clone this repository:

```bash
git clone <repository_url>
cd word-counter-cli
```

Build the project:

```bash
cargo build --release
```

Or run directly without building:

```bash
cargo run -- <file_path>
```

---

## Usage

### Basic Usage

Count lines, words, and characters for a file:

```bash
cargo run -- text.txt
```

Example output:

```
Lines: 100
Words: 450
Characters: 2300
Top 5 words:
1. the (20 times)
2. rust (15 times)
3. and (12 times)
4. code (10 times)
5. to (8 times)
```

### Top N Words

You can specify how many top words to display using `--top N`:

```bash
cargo run -- text.txt --top 10
```

This will display the 10 most frequent words instead of the default 5.

---

## Example

```bash
cargo run -- sample.txt --top 3
```

Output:

```
Lines: 50
Words: 300
Characters: 1500
Top 3 words:
1. rust (25 times)
2. code (18 times)
3. learning (15 times)
```

---

## Error Handling

- If the file is not found:

```
Error: File 'text.txt' not found
```

- Empty files or files without words are handled gracefully.
- Any I/O errors are propagated and printed in the terminal.

---

## Learning Focus

- **File I/O in Rust:** Using `BufReader`, `File`, and `OpenOptions`.
- **Command-line arguments:** Parsing file path and optional parameters.
- **Data structures:** Using `HashMap` for word frequency counting.
- **Iteration and collection:** `.iter()`, `.collect()`, `.sort_by()`.
- **Structs for grouping results:** Returning multiple values cleanly.
- **Error handling:** Using `Result<T, E>` idiomatically.

---

## License

MIT License © [Your Name]

