module Highlights
  class CLI
    Options = Struct.new(:filename, :output)

    def initialize(args)
      args << '-h' if ARGV.empty?
      @args = args
    end

    def run
      options = get_options
      document = Parser.new(options.filename).parse
      Renderer.render(document, options.output)
    end

    def get_options
      options = Options.new(nil, "notes.md")

      OptionParser.new do |opts|
        opts.banner = "Usage: highlights -f file.csv -o [output file]"

        opts.on("-fFILENAME", "--file=FILENAME", "Kindle notes CSV file") do |f|
          options.filename = f
        end

        opts.on("-oOUTPUT", "--output=OUTPUT", "Output file. Accepts HTML and markdown (default: notes.md)") do |o|
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
