# Highlights

A CLI that converts Kindle CSV documents to Markdown or HTML.

## Example

```
$ highlights -f zamm_notes.csv -o zamm_notes.md
$ head zamm_notes.md
```

Outputs:

```
# ZEN AND THE ART OF MOTORCYCLE MAINTENANCE: AN INQUIRY INTO VALUES

by Robert M. Pirsig

## Notes

> You�re a passive observer and it is all moving by you boringly in a frame.

Highlight (Yellow): Page 4

> Instead you spend your time being aware of things and meditating on them. On sights and sounds, on the mood of the weather and things remembered, on the machine and the countryside you�re in, thinking about things at great leisure and length without being hurried and without feeling you�re losing time.

Highlight (Yellow): Page 6
```

## Installation

```
$ gem install highlights
```

## Usage

```
$ highlights -f zamm_notes.csv
```

```
$ highlights -h

Usage: highlights -f file.csv -o [output file]
    -f, --file=FILENAME              Kindle notes CSV file
    -o, --output=OUTPUT              Output file. Accepts HTML and markdown (default: notes.md)
    -h, --help                       Show help
    -v, --version                    Show version
```

## Development

After checking out the repo, run `bin/setup` to install dependencies. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and tags, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/mgmarlow/highlights.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
