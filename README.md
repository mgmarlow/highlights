# highlights

Convert Kindle "My Clippings.txt" files into structured data.

Highlights for ebooks manually uploaded to a Kindle device are stored
in the on-device "My Clippings.txt" rather than the usual highlight
notebook. This CLI converts "My Clippings.txt" into JSON so you can
easily import it into other formats. Or, create a SQLite database that
you can query directly.

Shoutout to [Standard Ebooks](https://standardebooks.org/) for being the
reason I have so many highlights stored as Kindle clippings.

## Usage

```sh
cargo run -- --filename ~/Documents/My\ Clippings.txt --format json
```

### Options

- `--filename`: Path to Kindle clippings file (required)
- `--format`: Output format - `json`, `summary`, or `sqlite` (default:
  summary)
- `--outfile`: Write to file instead of stdout (optional, defaults to
  `highlights.db` for sqlite format)

## Outputs

### JSON Format

```json
[
  {
    "title": "Book Title",
    "author": "Book Author",
    "kind": "Highlight",
    "page": "42",
    "location": "1234-1235",
    "location_start": 1234,
    "location_end": 1235,
    "date": "Monday, March 4, 2024 5:17:31 PM",
    "content": "The highlighted text"
  }
]
```

### SQLite Format

The `sqlite` format creates a SQLite db with two tables:

- **highlights**: Same as the JSON format, but only stores Highlights (no
  bookmarks or notes).
- **notes** - Notes that point to a highlight_id reference.

Example query:

```sql
select notes.content, highlights.title
from notes
join highlights on highlights.id=notes.highlight_id;
```

## Build

```sh
cargo build --release
```
