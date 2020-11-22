module Highlights
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

        opts.on("-oOUTPUT", "--output=OUTPUT", "Output file (default: notes.md)") do |o|
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
