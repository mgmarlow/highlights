module Highlights
  class Renderer
    def initialize(document, outfile)
      @document = document
      @outfile = outfile
    end

    def render
      File.open(@outfile, 'w') do |file|
        file.puts("# #{@document.title}")
        file.puts("## #{@document.author}")
        file.write("\n")
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
end
