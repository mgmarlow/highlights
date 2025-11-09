# highlights

Convert Kindle "My Clippings.txt" files into structured JSON.

## Usage

```sh
cargo run -- --filename ~/Documents/My\ Clippings.txt --format json
```

### Options

- `--filename`: Path to Kindle clippings file (required)
- `--format`: Output format - `json` or `summary` (default: summary)
- `--outfile`: Write to file instead of stdout (optional)

## Output

Parses highlights, notes, and bookmarks into structured data:

```json
[
  {
    "title": "Book Title",
    "kind": "Highlight",
    "page": "42",
    "location": "1234-1235",
    "date": "Monday, March 4, 2024 5:17:31 PM",
    "content": "The highlighted text"
  }
]
```

## Build

```sh
cargo build --release
```
