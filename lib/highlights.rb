require "highlights/version"
require "csv"
require "optparse"

module Highlights
  class Error < StandardError; end
  Note = Struct.new(:type, :location, :starred, :annotation)
  Document = Struct.new(:title, :author, :notes)

  class Renderer
    def initialize(document, outfile)
      @document = document
      @outfile = outfile
    end

    def render
      File.open(@outfile, 'w') do |file|
        file.puts("# #{@document.title}")
        file.puts("## #{@document.author}")
        file.puts("### Notes")
        file.write("\n")

        @document.notes.each do |note|
          file.puts("> #{note.annotation}")
          file.write("\n")
          file.puts("#{note.type}: #{note.location}")
          file.write("\n")
        end
      end
    end
  end

  class Parser
    class MalformedCSVError < Error; end
    NOTES_STARTING_POSITION = 8

    def initialize(filename)
      @filename = filename
    end

    def parse
      table = CSV.read(@filename, liberal_parsing: true)

      notes = table[NOTES_STARTING_POSITION...].map do |note_row|
        Note.new(*note_row)
      end

      Document.new(table[1][0], table[2][0], notes)
    rescue StandardError => e
      raise MalformedCSVError.new(e.message)
    end
  end

  class CLI
    Options = Struct.new(:filename, :output)

    def initialize(args)
      @args = args
    end

    def run
      options = get_options
      document = Parser.new(options.filename).parse
      Renderer.new(document, options.output).render
    end

    def get_options
      options = Options.new(nil, "notes.md")

      OptionParser.new do |opts|
        opts.on("-fFILENAME", "--file=FILENAME", "Kindle notes CSV file") do |f|
          options.filename = f
        end

        opts.on("-oOUTPUT", "--output=OUTPUT", "Output file") do |o|
          options.output = o
        end

        opts.on("-h", "--help", "Show help") do
          puts opts
          exit
        end

        opts.on("-v", "--version", "Show version") do
          puts VERSION
          exit
        end
      end.parse!(@args)

      options
    end
  end
end
