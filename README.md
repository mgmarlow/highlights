# highlights

Convert Kindle "My Clippings.txt" files into structured JSON.

Highlights for ebooks manually uploaded to a Kindle device are
stored in the on-device "My Clippings.txt" rather than the usual
highlight notebook. This CLI reads this text file and converts
it to JSON so you can easily import it into other formats.

Shoutout to [Standard Ebooks](https://standardebooks.org/) for
being the reason I have so many highlights stored as Kindle
clippings.

## Usage

```sh
cargo run -- --filename ~/Documents/My\ Clippings.txt --format json
```

### Options

- `--filename`: Path to Kindle clippings file (required)
- `--format`: Output format - `json`, `summary`, or `sqlite` (default: summary)
- `--outfile`: Write to file instead of stdout (optional, defaults to `highlights.db` for sqlite format)

## Output

Parses highlights, notes, and bookmarks into structured data.

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

The `sqlite` format creates a SQLite3 db with two tables:

- **highlights**: Same as the JSON format, but only stores Highlights (no
  bookmarks or notes).
- **notes** - Notes that point to a highlight_id reference.

Example usage:

```sh
cargo run -- --filename ~/Documents/My\ Clippings.txt --format sqlite --outfile my_highlights.db
```

```sql
select notes.content, highlights.title
from notes
join highlights on highlights.id=notes.highlight_id;
```

## Build

```sh
cargo build --release
```
