module Highlights
  module Formatter
    module Markdown
      def render_markdown
        File.open(@outfile, 'w') do |file|
          file.puts("# #{@document.title}")
          file.write("\n")
          file.puts(@document.author)
          file.write("\n")
          file.puts("## Notes")
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
end
